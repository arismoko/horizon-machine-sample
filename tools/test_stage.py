#!/usr/bin/env python3
"""Audit the real pinned export, deterministic staging and unchanged checkout."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import horizon


def snapshot(root):
    return {str(p.relative_to(root)): (horizon.digest(p.read_bytes()), p.stat().st_mtime_ns)
            for p in root.rglob('*') if p.is_file()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--horizon', type=Path, default=horizon.ROOT.parent/'horizon')
    args = parser.parse_args()
    checkout = args.horizon.resolve()
    status = subprocess.check_output(['git', '-C', str(checkout), 'status', '--porcelain'])
    before = {p: (checkout/p).read_bytes() for p in horizon.COMPAT['source_sha256']}
    (horizon.ROOT/'target').mkdir(exist_ok=True)
    with horizon.lock(horizon.ROOT/'target/stage.lock'):
        with tempfile.TemporaryDirectory(prefix='native-stage-test-', dir=horizon.ROOT/'target') as scratch:
            root = Path(scratch)
            output, changed = horizon.trait_stage(checkout, root)
            assert changed
            first = snapshot(output)
            second, changed = horizon.trait_stage(checkout, root)
            assert second == output and not changed and first == snapshot(output)
            staged = root/'horizon-native-mods/machine_sample'
            manifest = horizon.tomllib.loads((staged/'horizon-mod.toml').read_text())
            assert manifest['native']['horizon'] == horizon.PIN
            assert 'injection' not in manifest['native']
            assert (staged/'src/machine.rs').read_bytes() == (horizon.ROOT/'src/machine.rs').read_bytes()
            assert (staged/'Cargo.toml').is_file()
            assert not any((staged/path).exists() for path in
                           ('native', 'rust', 'horizon-native.json', 'horizon-mixin.toml'))
            horizon.check_injections(output, staged/'horizon-mod.toml')
            broken = root/'broken-horizon-mod.toml'
            broken.write_text((staged/'horizon-mod.toml').read_text() + '''
[[inject]]
id = "deliberately_missing_slot"
at = "horizon_client::game::Game::sample_nonexistent_slot"
kind = "head"
access = "api"
with = "listener"
hash = "0000000000000000"
''')
            try:
                horizon.check_injections(output, broken)
            except subprocess.CalledProcessError as error:
                assert 'unknown API point' in error.stderr, error.stderr
                assert 'deliberately_missing_slot' in error.stderr, error.stderr
            else:
                raise AssertionError('canonical resolver accepted an impossible API injection')
            mismatched = root/'mismatched.lock'
            mismatched.write_text('[[package]]\nname = "libc"\nversion = "999.0.0"\n')
            try:
                horizon.check_lock(output/'Cargo.lock', mismatched)
            except ValueError as error:
                assert 'libc 999.0.0' in str(error)
            else:
                raise AssertionError('a mismatched shared dependency was accepted')
    assert all((checkout/p).read_bytes() == content for p, content in before.items())
    assert subprocess.check_output(['git', '-C', str(checkout), 'status', '--porcelain']) == status
    print(json.dumps({'status': 'ok', 'original_checkout_unchanged': True,
                      'flat_package_staged': True, 'canonical_package_writer': True,
                      'deterministic_source_bytes_and_mtimes': True}, indent=2))


if __name__ == '__main__':
    main()
