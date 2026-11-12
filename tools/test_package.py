#!/usr/bin/env python3
"""Exercise the real shared package writer and flat staging input."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import zipfile
import horizon


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--horizon', type=Path, default=horizon.ROOT.parent/'horizon')
    args = parser.parse_args()
    root = horizon.ROOT
    pin = (root / 'horizon-mod.toml').read_bytes()
    command = ['python3', str(root / 'tools/package.py'), '--horizon', str(args.horizon)]
    subprocess.run(command, check=True)
    package = root / 'target/horizon-mod/machine_sample.hmod'
    first = package.read_bytes()
    subprocess.run(command, check=True)
    assert package.read_bytes() == first
    with zipfile.ZipFile(package) as archive:
        names = set(archive.namelist())
        assert {'horizon-mod.toml', 'Cargo.toml', 'src/lib.rs', 'src/machine.rs'} <= names
        assert any(name.startswith('assets/machine_sample/') for name in names)
        assert any(name.startswith('data/machine_sample/') for name in names)
        assert not any(name.startswith(('native/', 'rust/', 'mixins/', 'api/', 'content/'))
                       for name in names)
        assert 'Cargo.lock' in names, 'native registry dependencies require a pinned lockfile'
        assert not {'horizon-native.json', 'horizon-mixin.toml', 'compatibility.json'} & names
    with tempfile.TemporaryDirectory(dir=root / 'target') as scratch:
        with horizon.lock(root/'target/stage.lock'):
            source, _ = horizon.stage(args.horizon.resolve(), Path(scratch)/'original')
            mods = horizon.stage_mods(source, Path(scratch))
        staged = mods / 'machine_sample'
        assert (staged / 'horizon-mod.toml').read_bytes() == pin
        assert (staged / 'src/lib.rs').read_bytes() == (root / 'src/lib.rs').read_bytes()
        assert not (staged / 'native').exists()
    assert (root / 'horizon-mod.toml').read_bytes() == pin
    print(json.dumps({'canonical_package': True, 'deterministic': True, 'flat_staging': True}))


if __name__ == '__main__':
    main()
