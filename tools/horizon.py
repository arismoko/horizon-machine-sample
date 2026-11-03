#!/usr/bin/env python3
"""Export pinned sources, stage the flat package and check its crate."""
import argparse
import contextlib
import fcntl
import hashlib
import json
import os
import tomllib
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
import zipfile

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = tomllib.loads((ROOT / "horizon-mod.toml").read_text())
COMPAT = json.loads((ROOT / "compatibility.json").read_text())
PIN = MANIFEST["native"]["horizon"]
# Matches launcher/src/rust_toolchain.rs at the pinned Horizon revision.
TOOLCHAIN = "1.99.1"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def stage(horizon, stages):
    if not (horizon / ".git").exists():
        raise ValueError("--horizon must be a Git checkout (including a worktree)")
    subprocess.run(["git", "-C", str(horizon), "cat-file", "-e", PIN + "^{commit}"], check=True, capture_output=True)
    inputs = {"revision": PIN.encode(), "export-schema": b"2"}
    signature = digest(b"".join(k.encode() + b"\0" + value for k, value in sorted(inputs.items())))
    stages.mkdir(parents=True, exist_ok=True)
    out = stages / signature[:20]
    if out.exists():
        marker = json.loads((out / ".horizon-mod-template.json").read_text())
        if marker["signature"] != signature:
            raise ValueError(f"stage marker mismatch: {out}")
        for path, expected in marker["files"].items():
            if digest((out / path).read_bytes()) != expected:
                raise ValueError(f"staged source was edited: {path}; remove this generated stage to rebuild it")
        return out, False
    temporary = Path(tempfile.mkdtemp(prefix=".mod-template-stage-", dir=stages))
    try:
        process = subprocess.Popen(["git", "-C", str(horizon), "archive", "--format=tar", PIN], stdout=subprocess.PIPE)
        try:
            subprocess.run(["tar", "-x", "-C", str(temporary)], stdin=process.stdout, check=True)
        finally:
            process.stdout.close()
            code = process.wait()
        if code:
            raise ValueError(f"git archive exited {code}")
        for path, expected in COMPAT["source_sha256"].items():
            if digest((temporary / path).read_bytes()) != expected:
                raise ValueError(f"pinned source hash mismatch: {path}")
        # Materialize the committed documentation symlink in the source snapshot.
        # No Rust source is edited here.
        for file in temporary.rglob("*"):
            if file.is_symlink():
                content = file.read_bytes()
                file.unlink()
                file.write_bytes(content)
        (temporary / ".horizon-revision").write_text(PIN + "\n")
        # The source tree itself is deterministic. Cargo outputs live elsewhere.
        files = {str(p.relative_to(temporary)): digest(p.read_bytes())
                 for p in sorted(temporary.rglob("*")) if p.is_file()}
        (temporary / ".horizon-mod-template.json").write_text(json.dumps(
            {"signature": signature, "horizon_revision": PIN, "files": files}, sort_keys=True, indent=2) + "\n")
        os.rename(temporary, out)
    except BaseException:
        shutil.rmtree(temporary, ignore_errors=True)
        raise
    return out, True


def stage_mods(source, stages):
    """Package with the pinned game's writer, then stage its validated flat input."""
    command = ["nice", "-n", "19", "cargo", "run", "-j", "4", "--locked", "--offline",
               "--manifest-path", str(source / "Cargo.toml"),
               "--target-dir", str(ROOT / "target/package-tool"),
               "-p", "cargo-horizon-mod", "--", "build"]
    subprocess.run(command, cwd=ROOT, check=True, stdout=subprocess.PIPE, text=True,
                   env=dict(os.environ, RUSTUP_TOOLCHAIN=TOOLCHAIN))
    package = ROOT / "target/horizon-mod" / f"{MANIFEST['id']}.hmod"
    mods = stages / "horizon-native-mods"
    shutil.rmtree(mods, ignore_errors=True)
    with zipfile.ZipFile(package) as archive:
        archive.extractall(mods / MANIFEST["id"])
    return mods


def check_lock(horizon_lock, sample_lock):
    """Refuse a shared dependency version absent from the pinned game lock."""
    with horizon_lock.open("rb") as file:
        packages = tomllib.load(file)["package"]
    versions = {}
    for package in packages:
        versions.setdefault(package["name"], set()).add(package["version"])
    with sample_lock.open("rb") as file:
        sample = tomllib.load(file)["package"]
    for package in sample:
        name, version = package["name"], package["version"]
        if name in versions and version not in versions[name]:
            raise ValueError(f"{name} {version} differs from pinned Horizon lock: {sorted(versions[name])}")


def trait_stage(horizon, stages):
    """Export the pinned game and stage its flat mod without changing game sources."""
    crate = tomllib.loads((ROOT / "Cargo.toml").read_text())
    patches = crate["patch"]["crates-io"]
    if (COMPAT["horizon_revision"] != PIN
            or crate["dependencies"]["horizon-client"]["rev"] != PIN
            or any(patch["rev"] != PIN for patch in patches.values())
            or MANIFEST["native"]["mixins"] != COMPAT["horizon_mixins_revision"]):
        raise ValueError("The manifest, crate and development provenance pins must match")
    output, changed = stage(horizon, stages / "original")
    check_lock(output / "Cargo.lock", ROOT / "Cargo.lock")
    stage_mods(output, stages)
    return output, changed


@contextlib.contextmanager
def lock(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a") as handle:
        fcntl.flock(handle, fcntl.LOCK_EX)
        yield


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["stage", "check", "build", "test"])
    parser.add_argument("--horizon", type=Path, default=ROOT.parent / "horizon")
    parser.add_argument("--target-dir", type=Path, default=ROOT / "target/horizon-cargo")
    args = parser.parse_args()
    horizon = args.horizon.resolve()
    target = args.target_dir.resolve()
    if target.is_relative_to(horizon):
        parser.error("Cargo target must be outside the original Horizon checkout")
    started = time.monotonic()
    with lock(ROOT / "target/stage.lock"):
        output, changed = trait_stage(horizon, ROOT / "target/trait")
        if target.is_relative_to(output) or output.is_relative_to(target):
            parser.error("Cargo target and generated source stage must be separate")
        report = {"stage": str(output), "stage_changed": changed,
                  "horizon_revision": PIN, "staging_seconds": time.monotonic()-started}
        if args.action != "stage":
            if args.action == "build":
                raise ValueError("trait mods must be linked with horizon-launcher native-build; see README.md")
            command = ["nice", "-n", "19", "cargo", args.action, "-j", "4",
                       "--locked", "--offline", "-p", "horizon-example-mod",
                       "--target-dir", str(target)]
            if args.action == "test":
                command.extend(["--", "--nocapture", "--test-threads=4"])
            with lock(target.parent / (target.name + ".mod-template.lock")):
                run_start = time.monotonic()
                result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True,
                                        env=dict(os.environ, RUSTUP_TOOLCHAIN=TOOLCHAIN))
            report.update(cargo_seconds=time.monotonic()-run_start, cargo_command=command,
                          cargo_exit=result.returncode)
            log = ROOT / "target" / f"horizon-{args.action}.log"
            log.write_text(result.stdout + result.stderr)
            report["cargo_log"] = str(log)
            if result.returncode:
                print(result.stderr[-12000:], file=sys.stderr)
                print(json.dumps(report), file=sys.stderr)
                return result.returncode
            report["linked_executable"] = False
        print(json.dumps(report, indent=2))
        return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        if isinstance(error, subprocess.CalledProcessError) and error.stderr:
            print(error.stderr, file=sys.stderr)
        print(f"Native mod: {error}", file=sys.stderr)
        raise SystemExit(1)
