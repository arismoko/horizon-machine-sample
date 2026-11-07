# Validation

The flat project pins fork Horizon
`53cc666df8f68c840fc0dd6f86d29c4600417d2d` and its matching Mixins revision
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

### Energy buffer

Complete a machine recipe. Its scoped ticking Part recovers 25 energy in the
saved `energy` field, up to 1,000. The canonical `ENERGY` capability exposes this
buffer on every face, with insertion/extraction limited to 100 per call. This
uses the game's built-in transactional energy provider, not a parallel store.
Read the block entity with `/data get block X Y Z` after a recipe, then save,
quit and reopen the world and read it again. Complete enough recipes to verify
the amount stops at 1,000. A capability consumer should be able to extract in a
transaction and observe no change when it aborts. This gameplay trigger and
save/load were not run at this pin. The unit test verifies charging the saved
NBT field and capacity saturation; transaction behavior belongs to the game.

## In-game proof scope

A linked dev build and an integrated or dedicated-server run are distinct from
source compilation. Do not carry the previous nested-layout sample's linked
and integrated-server machine results forward as evidence for a flat pin.
State the exact source and tested server side whenever reporting new gameplay
proof. No player data or Minecraft assets belong in this repository.
