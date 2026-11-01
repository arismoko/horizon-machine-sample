#!/usr/bin/env bash
# Export the pinned committed fork sources and add this example; never edit the checkout.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
horizon=${1:?Usage: prepare.sh /path/to/horizon-fork-checkout /path/to/new-export}
out=${2:?Usage: prepare.sh /path/to/horizon-fork-checkout /path/to/new-export}
pin=$(sed -n 's/.*"horizon_revision": "\([0-9a-f]*\)".*/\1/p' "$here/compatibility.json")
if [ -e "$out" ]; then echo "Export already exists: $out" >&2; exit 1; fi
git -C "$horizon" cat-file -e "$pin^{commit}"
mkdir -p "$out"
git -C "$horizon" archive --format=tar "$pin" | tar -x -C "$out"
mkdir "$out/trait-example"
cp -R "$here/Cargo.toml" "$here/driver" "$here/machine-hooks" "$out/trait-example/"
cp "$out/Cargo.lock" "$out/trait-example/Cargo.lock"
echo "$pin" > "$out/.horizon-revision"
printf 'Prepared export: %s\n' "$out"
