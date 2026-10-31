# Entity join hook example

This opt-in example logs item and experience orb joins without canceling them.
The hook receives the dimension, entity ID, kind, position, disk-load flag,
and a mutable cancellation decision. The hook is limited to the entity types
supported by [Horizon PR #5167](https://github.com/horizon-rust/horizon/pull/5167).
The PR is not merged. This example pins its head,
`2af60d68a2b838d53e7322c16a54174325b60dd7`, not the repository's normal pin.
Revalidate it after the upstream PR changes or merges.

## Activate and build

The commands below make a disposable copy of this committed repository. They
replace its bundle and pin only in that copy. The normal machine stays unchanged.
Fetch the PR head into the Horizon checkout first. Set `HORIZON_MIXINS` to
the revision in the example's `compatibility.json`, as in the main README.

```sh
git -C /path/to/horizon fetch origin pull/5167/head
export HORIZON_MIXINS=/path/to/horizon-mixins
cargo fetch --locked --manifest-path "$HORIZON_MIXINS/Cargo.toml"
work=/home/ari/.cache/horizon-sample-mod/entity-join-work
mkdir "$work"  # Fail if an earlier build already exists.
git archive HEAD | tar -x -C "$work"
cp "$work/examples/entity-join/compatibility.json" "$work/compatibility.json"
rm -rf "$work/mixins"
mkdir -p "$work/mixins"
cp -R "$work/examples/entity-join/bundle" "$work/mixins/entity_join_example"
python3 "$work/tools/horizon.py" build --horizon /path/to/horizon \
  --target-dir /home/ari/.cache/horizon-sample-mod/entity-join-target
```

Use a fresh work directory for each build. The tool exports committed Horizon
sources and applies the bundle with Horizon Mixins. It does not edit Horizon.

## Game test

Run the script with absolute paths to a compatible `horizon-ctl`, the built
client, prepared 26.3 resources, and this repository's `content` directory:

```sh
examples/entity-join/game-test.sh /path/to/horizon-ctl \
  /home/ari/.cache/horizon-sample-mod/entity-join-target/fast/horizon-client \
  /path/to/prepared/26.3 "$PWD/content"
```

The script creates a fresh game directory, summons an item and an experience
orb, and requires a hook log line at each summoned position. It prints the
retained log path.
A missing line fails the test. This verifies delivery, not cancellation or
disk reload behavior.
