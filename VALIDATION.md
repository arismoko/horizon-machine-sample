# Validation

The flat project pins fork Horizon
`57f0b36960bbe26915d1af3f4053d64d8ebbf0a8` and its matching Mixins revision
`5a64af938ec86b00d53d16a408aabc0038fb3e57`. This pin includes the host-area
trait regroup, the `horizon-mod-package` rename, and the attachment and registry
key runtime split. The sample's existing trait imports and Part implementations
compile unchanged against this API. It retains typed block-entity declarations,
scoped-Part subscriptions and flat layout, handles the Part registration
`Result`, and its `register` hook takes the registration context. The launcher owns the generated main and its mod and
inventory markers; the staging check lays the canonical flat package out at
`horizon-native-mods/<id>/`, where the engine validates id, version and side
and refuses startup on mismatch, with no record fallback. There is no
handwritten entry, packaging filter or compatibility fallback.

## Source and package checks

Before a pin is ready, check its committed source hashes and seed the development
`Cargo.lock` from that game's lockfile, then resolve offline. Shared dependency
versions must agree with the game. Run the formatting, clippy, unit, staging and
package commands documented in README.md. Exact-head results belong in the PR's
validation section and CI, not in a status table that becomes stale at each pin.

`tools/test_stage.py` verifies the original checkout is unchanged and repeated
exports have identical source bytes and mtimes. Both staging helpers use the
pinned game's canonical package writer. `tools/test_package.py` checks the flat
archive's contents, deterministic output and staging; development provenance,
Cargo.lock and legacy manifests are not shipped.

`tools/horizon.py check` compiles the trait crate against the exact pin. It does
**not** link a game executable. A successful paired item 11 reports pin, lock,
stage and check separately; linking is an additional, explicitly named proof.

Mods don't ship a toolchain file; Horizon's pinned toolchain builds them. The
helper and CI explicitly select the pinned launcher's Rust 1.99.1.

## Feature tour triggers

### Commands and messages

As an operator, run `/machine_sample announce Machine ready`. Every player should
receive `[Machine Sample / player] Machine ready`; the caller also receives
`Announcement delivered.` Run the same command without the slash from the
console, or from a command block/function, to exercise typed non-player sources.
The prefix names the original source even under `execute as`; feedback follows
vanilla silence and command-block track-output rules. A non-operator must not
be able to execute this level 2 command. An unrelated command must be untouched.
Unit tests cover literal routing and parsed message formatting. These gameplay
triggers are documented, not yet exercised at this pin.

### Side-typed data and asset reload

Override `data/machine_sample/machine_tour/processing.json` in a datapack with
`{"minecraft:cobblestone":80}` and run `/reload`. A newly inserted cobblestone
should now take 80 server ticks to process instead of the default 40. Its screen
progress bar should still reach the end exactly on completion. Processing-time
keys must be namespaced item ids and durations integers in 1..=10,000; other
inputs keep the 40 tick default. Changing the duration mid-cycle keeps the saved
progress and applies the new threshold on the next tick.

Override `assets/machine_sample/machine_tour/screen.json` in a resource pack with
a JSON string of at most 24 characters, for example `"Cooling sample"`. Reload
client assets (F3+T) and open the machine to see it above the player inventory.
This asset listener cannot read server data; the server listener cannot read
client assets. A malformed table, out-of-range duration or multiline/overlong
label must fail prepare without publishing a replacement. Restore the file and
reload to recover.

Tests cover both parsers and duration thresholds, not actual pack overrides or
client drawing. Those triggers have not been run at this pin. Files are read
only during prepare; ticks use the published table, never filesystem reads.

## In-game proof scope

A linked dev build and an integrated or dedicated-server run are distinct from
source compilation. Do not carry the previous nested-layout sample's linked
and integrated-server machine results forward as evidence for a flat pin.
State the exact source and tested server side whenever reporting new gameplay
proof. No player data or Minecraft assets belong in this repository.
