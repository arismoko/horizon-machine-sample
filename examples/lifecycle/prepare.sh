#!/usr/bin/env bash
# Prepare an isolated sample overlay; never edit the checkout.
set -eu
root=$(cd "$(dirname "$0")/../.." && pwd)
out=${1:?Usage: prepare.sh /path/to/new-overlay}
if [ -e "$out" ]; then echo "Overlay already exists: $out" >&2; exit 1; fi
mkdir -p "$out"
# Copy only the files needed by the staging tool and the machine bundle.
cp -R "$root/tools" "$root/mixins" "$out/"
cp "$root/examples/lifecycle/compatibility.json" "$out/"
cp "$root/examples/lifecycle/hooks/"*.rs "$out/mixins/machine_sample/"
cat "$root/examples/lifecycle/hooks/injections.toml" >> "$out/mixins/machine_sample/horizon-mixin.toml"
python3 - "$out" "$root" <<'PY'
import json
from pathlib import Path
import sys
root = Path(sys.argv[1])
compat = json.loads((root / 'compatibility.json').read_text())
p = root / 'mixins/machine_sample/horizon-mixin.toml'
s = p.read_text()
old = 'horizon_revision = "' + json.loads((Path(sys.argv[2]) / 'compatibility.json').read_text())['horizon_revision'] + '"'
new = 'horizon_revision = "' + compat['horizon_revision'] + '"'
if s.count(old) != 1:
    raise SystemExit('Expected exactly one machine manifest revision')
p.write_text(s.replace(old, new))
PY
printf 'Prepared overlay: %s\n' "$out"
