# Level random helper

This opt-in example calls `native_level_random` on the server tick of a
machine. It draws an integer from the overworld's random source and checks that
an unknown dimension returns `None`. It writes one log line per machine, then
stores a marker in its block entity NBT. This changes the world's random
sequence. Use a fresh test world.

The example requires [Horizon PR #5146](https://github.com/horizon-rust/horizon/pull/5146)
merge commit `5062f2259224d3610e96019dd6b69d7523aeb85e`, not the default pin.
The live game test must be repeated against this merged revision.

## Activate and build

Use a disposable clean checkout of this repository. Fetch Horizon main into your
Horizon checkout, and set `HORIZON_MIXINS` to the path of a Mixins checkout at
the revision in `compatibility.json`. Then run from the sample checkout:

```sh
git -C /path/to/horizon fetch origin main
cp examples/level-random/compatibility.json compatibility.json
cp examples/level-random/horizon-mixin.toml mixins/machine_sample/horizon-mixin.toml
cp examples/level-random/tick.rs mixins/machine_sample/tick.rs
python3 tools/horizon.py build --horizon /path/to/horizon --target-dir "$HOME/.cache/horizon-sample-mod/level-random-target"
```

These copies activate the example only in this checkout. Discard them after the
test; the default machine does not call this helper. The staging tool exports
the pinned committed Horizon source and does not edit the Horizon checkout.

## Game test

The build above produces only `horizon-client`. Build `horizon-ctl` separately
from the same pinned Horizon revision, for example from the generated source
stage after the build:

```sh
cargo build --manifest-path target/mixins/modded/Cargo.toml --locked --offline \
  -p horizon-client --bin horizon-ctl --no-default-features --profile fast \
  --target-dir "$HOME/.cache/horizon-sample-mod/level-random-target"
```

Use that `horizon-ctl`, the built client, and prepared 26.3 resources:

```sh
CTL="$HOME/.cache/horizon-sample-mod/level-random-target/fast/horizon-ctl" CLIENT="$HOME/.cache/horizon-sample-mod/level-random-target/fast/horizon-client" RESOURCES=/path/to/26.3 examples/level-random/game-test.sh
```

The script creates a fresh game directory, places a machine, and requires a
log line with an integer draw and `invalid_none=true`. A missing line or a
failed assertion fails the script. The printed directory retains the log for
inspection. This test exercises the live server hook; a build alone does not.

On `lt-24-005`, the test passed at sample commit `56a07a2063fe5731f82673ebcb3624fc79021c0e`
with Horizon `5062f2259224d3610e96019dd6b69d7523aeb85e`. The script exited 0
and logged `overworld=Some(-2051041063) invalid_none=true`.
