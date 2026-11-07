# Validation

The flat project pins fork Horizon
`cb01ae304c7136f1598f97650c63ccfc1da47b90` and its matching Mixins revision
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

### Client inspection and screen overlay

Look at a sample machine with no screen open and press I. The view should switch
from first person to third person behind, and a local UI click should play.
With the machine still targeted, press again to return; vanilla F5 also remains
available. Looking at another block, an entity or empty space must do nothing,
as must holding the key (repeat events). Rebind Inspect machine view under
Options > Controls and confirm only the new binding works. Open the machine
screen and inspect the `40 tick cycle` overlay above its two slots; the Deposit
button and progress bar must remain unobscured. No other player should hear the
click or have their camera changed.

The unit test covers the camera toggle policy, not rendering, sound or input
routing. These visual/audio triggers have not been exercised at this pin and
need a reviewer client run.

### Fluid tank and buckets

As an operator holding a water bucket in the main hand, run
`/machine_sample fill X Y Z` targeting a sample machine in your dimension.
Expect `Transferred 1000 mB.` and an empty bucket in place of the water bucket.
Hold the empty bucket and run `/machine_sample drain X Y Z` to reverse the move.
Fill four times, then verify a fifth fill moves zero and leaves the held bucket
unchanged. Try lava while the tank contains water: no mixing or bucket loss.
Save and reopen with a nonempty tank and drain it to check persistence. A
non-machine or unloaded target must refuse the transfer, and console sources
must receive the player-only explanation. The operator commands intentionally
allow a target anywhere loaded in the player's current dimension.

This reuses the game's transactional `FLUIDS` and `ITEM_FLUIDS` providers and
`move_resources`, not handwritten tank or bucket NBT. Unit tests cover the tank
configuration and command routing. Gameplay and save/load triggers have not
been exercised at this pin.

## In-game proof scope

A linked dev build and an integrated or dedicated-server run are distinct from
source compilation. Do not carry the previous nested-layout sample's linked
and integrated-server machine results forward as evidence for a flat pin.
State the exact source and tested server side whenever reporting new gameplay
proof. No player data or Minecraft assets belong in this repository.
