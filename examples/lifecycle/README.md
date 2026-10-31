# Chunk and player lifecycle hooks

This opt-in example logs chunk loads and unloads and player login and logout.
The four hook helpers receive the dimension and chunk coordinates, or the
player UUID and name. Replace the log statements with your own behavior.
Chunk load's `new` flag distinguishes newly generated chunks.

The example requires the unmerged [Horizon PR #5131](https://github.com/horizon-rust/horizon/pull/5131)
head `815de4119d2f984e2c97e67c163c705cc79b7be4`. It does not work at the
repository's default pin. This is a draft example; revalidate it against the
merged revision before relying on it. The default machine bundle stays unchanged.

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
