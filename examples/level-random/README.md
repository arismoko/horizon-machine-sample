# Level random helper

This opt-in example calls `native_level_random` on the server tick of a
machine. It draws an integer from the overworld's random source and checks that
an unknown dimension returns `None`. It writes one log line per machine, then
stores a marker in its block entity NBT. This changes the world's random
sequence. Use a fresh test world.

The example requires [Horizon PR #5146](https://github.com/horizon-rust/horizon/pull/5146)
head `ce12d07965e337040611534577834438be58c65c`, not the default pin.
It is unmerged; revalidate against the merged revision before use.

## Activate and build

Use a disposable clean checkout of this repository. Fetch the PR head into your
Horizon checkout, and set `HORIZON_MIXINS` to the revision in
`compatibility.json`. Then run from the sample checkout:

```sh
git -C /path/to/horizon fetch origin pull/5146/head
cp examples/level-random/compatibility.json compatibility.json
cp examples/level-random/horizon-mixin.toml mixins/machine_sample/horizon-mixin.toml
cp examples/level-random/tick.rs mixins/machine_sample/tick.rs
python3 tools/horizon.py build --horizon /path/to/horizon --target-dir /home/ari/.cache/horizon-sample-mod/level-random-target
```

These copies activate the example only in this checkout. Discard them after the
test; the default machine does not call this helper. The staging tool exports
the pinned committed Horizon source and does not edit the Horizon checkout.

## Game test

Use a compatible `horizon-ctl`, the built client, and prepared 26.3 resources:

```sh
CTL=/path/to/horizon-ctl CLIENT=/home/ari/.cache/horizon-sample-mod/level-random-target/fast/horizon-client RESOURCES=/path/to/26.3 examples/level-random/game-test.sh
```

The script creates a fresh game directory, places a machine, and requires a
log line with an integer draw and `invalid_none=true`. A missing line or a
failed assertion fails the script. The printed directory retains the log for
inspection. This test exercises the live server hook; a build alone does not.
