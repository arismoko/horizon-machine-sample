#!/usr/bin/env bash
# Prepare an isolated sample overlay; never edit the checkout.
set -eu
root=$(cd "$(dirname "$0")/../.." && pwd)
out=${1:?Usage: prepare.sh /path/to/new-overlay}
if [ -e "$out" ]; then echo "Overlay already exists: $out" >&2; exit 1; fi
mkdir -p "$out"
# Copy only the files needed by the staging tool and the machine bundle.
cp -R "$root/tools" "$root/mixins" "$out/"
cp "$root/compatibility.json" "$out/"
cp "$root/examples/lifecycle/hooks/"*.rs "$out/mixins/machine_sample/"
cat "$root/examples/lifecycle/hooks/injections.toml" >> "$out/mixins/machine_sample/horizon-mixin.toml"
python3 - "$out" <<'PY'
import json
from pathlib import Path
import sys
root = Path(sys.argv[1])
revision = '815de4119d2f984e2c97e67c163c705cc79b7be4'
p = root / 'compatibility.json'
data = json.loads(p.read_text())
data['horizon_revision'] = revision
data['source_sha256'] = {
    'crates/horizon-client/src/game.rs': '825a21600d76dfbba7a0984534f0f154ae68902e7bb528dbab9cb1d695edadd9',
    'crates/horizon-client/src/game/native_hooks.rs': 'f39869fe1565a7b1ce3d514b8b5b45bff35397bb9a09e456eeb17073bc108f4d',
}
p.write_text(json.dumps(data, indent=2) + '\n')
p = root / 'mixins/machine_sample/horizon-mixin.toml'
s = p.read_text().replace('36f4bc00d6fc644f1ad118ddd926a997a034cb48', revision)
p.write_text(s)
PY
printf 'Prepared overlay: %s\n' "$out"
