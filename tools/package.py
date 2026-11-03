#!/usr/bin/env python3
"""Package native code and top-level content using Horizon's shared .hmod writer."""
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def package():
    subprocess.run(['cargo', 'horizon-mod', 'build'], cwd=ROOT, check=True)
    manifest = tomllib.loads((ROOT / 'horizon-mod.toml').read_text())
    out = ROOT / 'target/horizon-mod' / f"{manifest['id']}.hmod"
    print(json.dumps(dict(file=str(out), size=out.stat().st_size,
                          sha256=hashlib.sha256(out.read_bytes()).hexdigest()), indent=2))


if __name__ == '__main__':
    package()
