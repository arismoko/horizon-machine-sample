#!/usr/bin/env python3
"""Exercise the real shared package writer and native-to-tool staging mapping."""
import json
from pathlib import Path
import subprocess
import tempfile
import zipfile
import horizon


def main():
    root = horizon.ROOT
    pin = (root / 'compatibility.json').read_bytes()
    subprocess.run(['python3', str(root / 'tools/package.py')], check=True)
    package = root / 'target/horizon-mod/machine_sample.hmod'
    first = package.read_bytes()
    subprocess.run(['python3', str(root / 'tools/package.py')], check=True)
    assert package.read_bytes() == first
    with zipfile.ZipFile(package) as archive:
        names = set(archive.namelist())
        assert {'horizon-mod.toml', 'horizon-native.json', 'native/horizon-mixin.toml',
                'native/rust/Cargo.toml', 'native/rust/src/lib.rs'} <= names
        assert any(name.startswith('assets/machine_sample/') for name in names)
        assert any(name.startswith('data/machine_sample/') for name in names)
        assert not any(name.startswith(('mixins/', 'api/', 'content/', 'native/machine_sample/'))
                       for name in names)
    with tempfile.TemporaryDirectory(dir=root / 'target') as scratch:
        mods = horizon.stage_mods(Path(scratch))
        staged = mods / 'machine_sample'
        assert (staged / 'horizon-mixin.toml').read_bytes() == (root / 'native/horizon-mixin.toml').read_bytes()
        assert (staged / 'rust/src/lib.rs').read_bytes() == (root / 'native/rust/src/lib.rs').read_bytes()
        assert not (staged / 'native').exists()
    assert (root / 'compatibility.json').read_bytes() == pin
    print(json.dumps({'canonical_package': True, 'deterministic': True, 'tool_mapping': True}))


if __name__ == '__main__':
    main()
