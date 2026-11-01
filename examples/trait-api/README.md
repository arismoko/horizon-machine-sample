# Trait mod API example

This opt-in example implements the generated `horizon_client::mod_api::Hooks`
trait with one small, machine-flavoured behaviour per hook. It does not use
horizon-mixins and does not change the default machine bundle. It is a draft
API: it needs the fork commit below, which is not in upstream Horizon.

The pin is `affbb5faaa40816ed01a68a89ff5a446ce583d23` on branch `mod-api` of
[arismoko/horizon](https://github.com/arismoko/horizon), recorded in
`compatibility.json`. A small executable (`driver`) links the mod and calls
`horizon_client::entry::main_with(mods)`.

## The four hooks

Hooks take `&self` and the mod must be `Send + Sync`. One `MachineHooks` value
serves both the window's game (client) and the integrated server's game
(server). Each hook runs on one side only, and its text says which.

| Hook | Side | Behaviour |
| --- | --- | --- |
| `level_tick_pre` | server | Counts ticks for each level and logs `MACHINE_SAMPLE server clock` every 200 ticks. |
| `zoom` | client | A lens: zooms only when `MACHINE_SAMPLE_ZOOM=<factor>` (above 1) is set when the client starts. |
| `tooltip` | client | Adds a `[client]` recipe note to cobblestone and stone, the items of the sample machine recipe. |
| `use_item_on` | server | A stick used on a `machine_sample:machine` block shows the acting player a `[server]` action-bar line with the block state and cancels the use. Other uses continue. |

Limits at this pin: `Host` has no key state, so the zoom hook cannot read keys; it uses a start-up setting instead.
`use_item_on` reads the block through `Host::block`; the exact block-state text
for the content mod's machine is checked by the game test below, not by unit
tests. The machine block exists only with the sample content folder loaded.

## Build

Use a Git checkout of the fork that contains the pinned commit. The script
exports only that committed revision into a new folder and adds this example
next to it. It never edits your checkout. Use new folders under the cache:

```sh
C=/home/ari/.cache/horizon-sample-mod
examples/trait-api/prepare.sh /path/to/horizon-fork "$C/trait-api-export"
cd "$C/trait-api-export/trait-example"
export CARGO_TARGET_DIR="$C/trait-api-target"
cargo test --offline -p machine-hooks
cargo build --offline --profile fast -p machine-hooks-driver
```

The first build compiles the whole client and takes a long time. The
executable is `$CARGO_TARGET_DIR/fast/machine-hooks-driver`. Remove the export
folder before preparing it again. Nothing from the export is committed.

## Run the integrated-server game test

Use prepared 26.3 resources and a fresh game folder. The native mod runs with
the player's rights; only run code you trust.

```sh
MACHINE_SAMPLE_ZOOM=3 "$C/trait-api-target/fast/machine-hooks-driver" \
  --resources /path/to/prepared/26.3 --game-dir /path/to/a/fresh/game \
  --dev-mod "$PWD/content" --world TraitApi --creative
```

Run it from the repository root so `$PWD/content` is the sample content.
Then check each hook in the world:

1. **Server clock.** After about 10 seconds the game log shows
   `MACHINE_SAMPLE server clock: 200 ticks in minecraft:overworld`.
2. **Zoom (client).** The view is zoomed 3 times from the start. Restart
   without `MACHINE_SAMPLE_ZOOM` and the view is normal.
3. **Tooltip (client).** Open the creative inventory and hover over cobblestone
   or stone. A `[client]` note appears under the name.
4. **Machine read (server).** Place a `machine_sample:machine` block, hold a
   stick, sneak and right-click the machine. The hook runs only when the block
   lets the click through. The action bar shows a `[server] read #1 ...` line
   (`Host::tell` does not reach the acting player inside their own hook
   until upstream #5677).
   Without sneaking, the block handles the click first.

This trait-only build has no menu hooks yet (planned for a later API step), so
the machine's screen never opens: ordinary use reports that
`machine_sample:machine` is not available yet. The machine screen, recipe and
pipe need the mixin bundle; see the [root README](../../README.md), which is
the full example.
