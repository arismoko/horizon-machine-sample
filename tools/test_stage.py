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
            manifest = horizon.tomllib.loads((root/'mods/machine_sample/horizon-mixin.toml').read_text())
            assert manifest['api']['path'] == 'rust'
            assert 'injection' not in manifest and 'crate' not in manifest
            assert (root/'mods/machine_sample/rust/src/machine.rs').read_bytes() == (horizon.ROOT/'native/rust/src/machine.rs').read_bytes()
    assert all((checkout/p).read_bytes() == content for p, content in before.items())
    assert subprocess.check_output(['git', '-C', str(checkout), 'status', '--porcelain']) == status
    print(json.dumps({'status': 'ok', 'original_checkout_unchanged': True,
                      'trait_bundle_staged': True, 'deterministic_source_bytes_and_mtimes': True}, indent=2))


if __name__ == '__main__':
    main()
