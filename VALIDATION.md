# Validation

The flat project pins fork Horizon
`e9f8d7991ef8e7b050e96364ce7c5b145a60116d` and its matching Mixins revision
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

## In-game proof scope

A linked dev build and an integrated or dedicated-server run are distinct from
source compilation. Do not carry the previous nested-layout sample's linked
and integrated-server machine results forward as evidence for a flat pin.
State the exact source and tested server side whenever reporting new gameplay
proof. No player data or Minecraft assets belong in this repository.
