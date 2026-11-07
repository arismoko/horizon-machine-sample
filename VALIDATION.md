# Validation

The flat project pins fork Horizon
`b357cb326e203200b345601edf2966322331eaf0` and its matching Mixins revision
`5a64af938ec86b00d53d16a408aabc0038fb3e57`. This pin includes the host-area
trait regroup, the `horizon-mod-package` rename, and the attachment and registry
key runtime split, plus API 2c's access declarations and generated host
forwarders, plus API 2d's per-kind registrar and capability-provider traits.
It also includes API 3a's owned nonempty `ItemStack` conversion boundary and
API 3d's native-only raw renderer split. The sample uses keyed item metadata,
component-aware merging, consuming shrink/split and fallible count growth.
Prepare validates typed item identifiers; apply replaces the duration table and
its key cache. The first server lookup resolves the table once, with one warning
per missing item per reload; subsequent ticks only read the keyed cache. Attachment tests use
resolved keys through `FakeHost`. Existing screen drawing stays on the API
`DrawContext` route; no raw render listener is added here.

It retains typed block-entity declarations, scoped-Part subscriptions and flat
layout, handles the Part registration
`Result`, and its `register` hook takes the registration context. The launcher owns the generated main and its mod and
inventory markers; the staging check lays the canonical flat package out at
`horizon-native-mods/<id>/`, where the engine validates id, version and side
and refuses startup on mismatch, with no record fallback. There is no
handwritten entry, packaging filter or compatibility fallback.

## Source and package checks

Before a pin is ready, check its committed source hashes and seed the development
`Cargo.lock` from that game's lockfile, then resolve offline. Shared dependency
versions must agree with the game. Run the formatting, clippy, unit, staging and
package commands documented in [Development](docs/DEVELOPMENT.md). Exact-head results belong in the PR's
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

Except for the separately pinned injection proof below, gameplay observations
in this section refer to the recorded linked proof at
`5d27928c5ce0ca8c041b0856ef56bfe5430bb6fc`, not a new game run at the current
source pin. A new pin's source/package checks do not refresh these observations.

### API-typed function-slot injection

The native `[[inject]]` declaration names the head of
`horizon_client::game::Game<'_>::on_tick_rate_changed`, not a hook or a raw
receiver. Its crate-root `tick_rate_seen` function takes `HostSide` and
`&mut OnTickRateChangedArgs`. It broadcasts only with the server Host; it does
not change the record's cancellation flag, so vanilla's autosave-wait adjustment
continues. There is no polling or extra work on ordinary ticks.

At `0a3b486ed7f2fa9c0eccaab141c9bc06306baaee`, the generated `points.json`
admits this target with a both-side writing Host and no arguments. The paired
`internals.json` supplies its hash `401a155ce7673f0b`; `points.json` itself has
no hash field. Copy that pin's hash, not one from another build. The focused
binding test checks the listener against the generated head-slot constructor.

As an operator, run `/tick rate 10`. Expect
`Machine Sample: tick-rate update observed.` alongside vanilla's tick-rate
response. Restore normal speed with `/tick rate 20`; the message should appear
again. Verified in a linked Developer build at
`e8e8109dc3f1c3624187ee425880aa6905d58693`: the pinned launcher accepted the
slot and hash, compiled the generated native binding, and installed
`native-e8e8109-6733542a5b5e`. In a fresh integrated-server world, both commands
produced the message on the server thread and in the client's chat history;
20 TPS was restored before quitting. The injection image shows those two
updates, not a wasm or dedicated-server run. Other screenshots retain the older
proof pin above.

This replaces a head mixin on vanilla `MinecraftServer.onTickRateChanged`;
[Horizon #1705](https://github.com/arismoko/horizon/issues/1705) requests a hook
for that moment. Hooks and Parts are preferred when one already exists.
For flavour 1 wasm, the same API `[[inject]]` shape supplies a listener from
`mod.wasm`, with `[wasm]` replacing `[native]` and generated guest exports.
No wasm artifact or wasm run is claimed here.

### Commands and messages

As an operator, run `/machine_sample announce Machine ready`. Every player should
receive `[Machine Sample / player] Machine ready`; the caller also receives
`Announcement delivered.` Run the same command without the slash from the
console, or from a command block/function, to exercise typed non-player sources.
The prefix names the original source even under `execute as`; feedback follows
vanilla silence and command-block track-output rules. A non-operator must not
be able to execute this level 2 command. An unrelated command must be untouched.
Unit tests cover literal routing and parsed message formatting. These gameplay
triggers are documented; the player announcement and its typed player prefix
were exercised at the recorded gameplay revision, but the non-player and
permission cases were not.

### Side-typed data and asset reload

The client label appears above the machine panel, clear of its title,
controls and inventory heading.

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

Tests cover both parsers and duration thresholds, not actual pack overrides.
The default label was viewed above the machine panel in the linked game;
pack overrides, F3+T and malformed-reload recovery have not been run at the
recorded gameplay revision. Files are read only during prepare; ticks use the
published table, never filesystem reads.

### Energy buffer

Complete a machine recipe. Its scoped ticking Part recovers 25 energy in the
saved `energy` field, up to 1,000. The canonical `ENERGY` capability exposes this
buffer on every face, with insertion/extraction limited to 100 per call. This
uses the game's built-in transactional energy provider, not a parallel store.
Read the block entity with `/data get block X Y Z` after a recipe, then save,
quit and reopen the world and read it again. Complete enough recipes to verify
the amount stops at 1,000. A capability consumer should be able to extract in a
transaction and observe no change when it aborts. The saved value reached 1,000
after processing a full stack and remained 1,000 after save/reopen. Capability
consumer and abort behavior were not exercised. The unit test verifies charging the saved
NBT field and capacity saturation; transaction behavior belongs to the game.

### Client inspection and screen overlay

Look at a sample machine with no screen open and press I. The view should switch
from first person to third person behind, and a local UI click should play.
With the machine still targeted, press again to return; vanilla F5 also remains
available. Looking at another block, an entity or empty space must do nothing,
as must holding the key (repeat events). Rebind Inspect machine view under
Options > Controls and confirm only the new binding works. Open the machine
screen and inspect the `Processing cycle` overlay above its two slots; the Deposit
button and progress bar must remain unobscured. No other player should hear the
click or have their camera changed.

The unit test covers the camera toggle policy, not rendering, sound or input
routing. The overlay and keybinding row were viewed in the linked game. Actual
physical-key camera switching, remapping and audio were not verified; control-tool
key taps are not evidence that the physical input hook fired.

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
configuration and command routing. Fill and drain each transferred 1,000 mB in
the linked game; authoritative tank data and the client's synchronized bucket
replacement were checked after allowing game ticks to advance. A nonempty tank
survived save/reopen. Capacity, mixing, abort and refusal cases were not exercised.

### Tags and gamerules

Put cobblestone in a container west of an empty machine. The default
`machine_sample:automatic_input` gamerule is true, so it should pull as before.
Run `/gamerule machine_sample:automatic_input false`: automatic intake must
stop, but a manually deposited input must still process and output normally.
Set it back to true to resume intake. Save/reopen with it false to check that
the registered rule persists with the world's gamerules.

The processing recipe now uses `#machine_sample:processable`, defined under
`data/machine_sample/tags/item/processable.json` with cobblestone as its sole
default member. A datapack can add granite to that tag; after `/reload`, granite
should also process into stone. Removing the override should restore the default
membership on reload. The unit test checks that the bundled recipe references
the bundled tag and its default contents. Runtime tag reload and gamerule
persistence have not been exercised at the recorded gameplay revision.

### Persistent entity attachments

Crouch and right-click a cow with an empty main hand. Its action-bar inspection
count should begin at 1 and increase on subsequent inspections. Inspect a
second cow to check that each entity has its own count. Save, quit, reopen and
inspect the first cow again: its count must continue rather than reset. Killing
it and summoning another cow must start a new count. Ordinary clicks, held-item
interactions and interactions without crouching must retain vanilla behavior.
The inspection gesture is claimed only after its attachment write succeeds.
Hold a non-using item such as a stick in the main hand while the off hand is
empty, then crouch and interact: the off-hand callback must not increment the
count. A unit regression test covers main/used-hand occupancy combinations.

`machine_sample:inspection_visits` has an NBT codec, no client sync, and no
copy-on-death policy. No entity handle or holder id is kept between callbacks.
Missing holders are ignored; malformed saved counts are refused without
replacing the value. Unit tests cover codec round trips through u32::MAX and
refusal of missing, wrong-typed, negative and oversized counts. Automated crouched
interactions displayed the count in the action bar. After saving at 8 and fully
reopening the world, the same cow was assigned a new runtime entity id and its
next automated interaction displayed 10 rather than restarting. These captures
establish persistence and feedback, not exactly one increment per physical click.
Per-entity isolation, death behavior and the held-item gesture guard were not
manually verified.

## In-game proof scope

The README pictures were captured at Horizon
`5d27928c5ce0ca8c041b0856ef56bfe5430bb6fc`, with sample code
including the hand guard from #40 and screen-label fix from #42 (the latter's
source head was `dc5d2616f17a7101cd445be3504dfa474060253e`). The exact-pin fork
launcher built and linked the sample in Developer mode: Cargo's `fast` profile
with the `horizon-client` package's optimization level set to 0. It ran offscreen
on Xvfb with software Vulkan/llvmpipe and a local integrated server, not a LAN
guest or dedicated server.

The source chest accepted 64 cobblestone; the output chest contained 64 stone.
The machine screen showed its input, live progress, overlay and unobscured reload
label. Its energy value reached 1,000. Tank fill/drain each reported 1,000 mB and
changed authoritative fluid data; a resumed-game check waited 61 ticks and
confirmed the client's replaced bucket matched the server. Energy and a nonempty
tank survived save/reopen. The cow's inspection counter also continued after a
full restart with a different runtime id. The player announcement's prefix and
feedback, and the registered keybinding row, were viewed.

This does not verify all triggers above. In particular, no dedicated server,
LAN guest, physical keyboard/audio path, pack override, transaction abort,
gamerule-false persistence or death-copy policy is claimed. The automation
explicitly closes screens before state-sensitive checks; a command succeeding
while paused is not proof that the client has processed the resulting state.

The resource inputs were private copies; the pinned vanilla exporter refreshed
fluid metadata required by this game revision. No original checkout or player
resources were changed. Only cropped ordinary gameplay images are committed,
not prepared tables, standalone vanilla assets or player saves.
