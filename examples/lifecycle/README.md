# Chunk and player lifecycle hooks

This opt-in example logs chunk loads and unloads and player login and logout.
The four hook helpers receive the dimension and chunk coordinates, or the
player UUID and name. Replace the log statements with your own behavior.
Chunk load's `new` flag distinguishes newly generated chunks.

The isolated overlay pins Horizon `75ba9ad839abecceba07a7583ce37964424b8033`,
which includes [Horizon PR #5131](https://github.com/horizon-rust/horizon/pull/5131).
The default machine bundle and its pin stay unchanged. The overlay pin and
source hashes are recorded in `examples/lifecycle/compatibility.json`.

## Build the example

Use a Horizon Git checkout containing that commit and a clean horizon-mixins
checkout at the revision in the root `compatibility.json`. The staging tool
exports committed Horizon sources and checks their hashes. It never edits the
Horizon checkout. Use a new overlay and a separate Cargo target directory:

```sh
export HORIZON_MIXINS=/path/to/pinned/horizon-mixins
export OVERLAY=/home/ari/.cache/horizon-sample-mod/lifecycle-overlay
examples/lifecycle/prepare.sh "$OVERLAY"
python3 "$OVERLAY/tools/horizon.py" build --horizon /path/to/horizon \
  --target-dir /home/ari/.cache/horizon-sample-mod/lifecycle-target
```

The overlay combines the machine bundle with the four injections. Remove the
overlay before preparing it again. Do not copy these hooks into the normal
bundle unless you want their log output on every game run.

## Run the game test

Use absolute paths, prepared 26.3 resources, and a compatible `horizon-ctl`.
The test creates a fresh game directory, teleports the player to force chunk
turnover, quits, and fails if any of the four event lines is absent. Keep the
control log to inspect the values. It does not prove exact event counts or order.

```sh
CTL=/path/to/horizon-ctl \
CLIENT=/home/ari/.cache/horizon-sample-mod/lifecycle-target/fast/horizon-client \
RESOURCES=/path/to/prepared/26.3 CONTENT="$PWD/content" \
TEST_DIR=/home/ari/.cache/horizon-sample-mod/lifecycle-game-test \
  examples/lifecycle/game-test.sh
```
