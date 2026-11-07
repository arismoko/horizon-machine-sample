# Developing and packaging the sample

The [README](../README.md) is the short tour. This page covers the build inputs
and the parts of the sample that are deliberately small.

## Exact build inputs

[horizon-mod.toml](../horizon-mod.toml) is the sole mod manifest. Its `[native]`
section pins the game and exact Horizon and horizon-mixins revisions. The
standalone API and points dependencies in [Cargo.toml](../Cargo.toml) must name that same Horizon
commit. [compatibility.json](../compatibility.json) records the source revision
and hashes used by the development exporter; it is not a runtime manifest.

Use a fork launcher that understands this API and package layout. Native builds
must match the installed base build's Horizon revision; an arbitrary upstream
release or a different Nightly is not a substitute. A stale launcher can also
stage obsolete crate names even when the mod itself compiles. The current CLI's
Developer mode currently uses Cargo's `fast` profile with the `horizon-client`
package's optimization level set to 0. Balanced and Optimized are separate
build modes. The README's native-build command uses Developer deliberately.

The pinned launcher brings its own Rust compiler. Local checks use Rust 1.99.1,
matching this pin. Mods do not ship a toolchain file. Cargo's cache must contain
the pinned dependencies before using `--offline`.

## Local checks

With a Git checkout containing the committed Horizon pin:

```sh
cargo +1.99.1 fmt --all --check
cargo +1.99.1 clippy --workspace --all-targets --locked --offline -- -D warnings
cargo +1.99.1 test --workspace --locked --offline
python3 tools/test_stage.py --horizon /path/to/horizon
python3 tools/horizon.py check --horizon /path/to/horizon
```

The stage test exports only committed pinned sources into ignored `target/`
trees. It verifies source hashes and shared dependency versions, stages the
canonical flat package, and leaves the original checkout unchanged. Repeated
exports must preserve source bytes and mtimes. The check compiles the trait
crate, not a linked executable; use the launcher's native build for linking and
actual gameplay. [VALIDATION](../VALIDATION.md) gives the proof scope and triggers.

Seed the development `Cargo.lock` from the pinned game's committed lockfile
before resolving the standalone crate. The launcher uses the game's lockfile
when staging the mod. The development lockfile is not packaged.

## Package layout

One flat project becomes one `.hmod`:

```text
horizon-mod.toml
Cargo.toml
src/lib.rs
src/machine.rs
src/tour/
assets/machine_sample/
data/machine_sample/
```

The mod's id and version come from `horizon-mod.toml`, not Cargo's package name
and version. `#[mod_api::horizon_mod]` exports its Hooks entry. The launcher owns
the generated game entry and tool manifest; this project has no raw injections
or handwritten factories. The API-typed `[[inject]]` declaration names a public
crate-root function and its generated `mod_points` record; the launcher owns
registration. It needs neither a `Points` implementation nor a handwritten
`entry::main_with` call. Its exact target and record come from the pinned game's
`points.json`, and its hash comes from the paired `internals.json`. This sample
leaves cancellation unchanged and uses no internals. Legacy nested layouts and
manifests are not accepted.

Package using the pinned game's shared writer:

```sh
python3 tools/package.py --horizon /path/to/horizon
python3 tools/test_package.py --horizon /path/to/horizon
```

The output is `target/horizon-mod/machine_sample.hmod`. Development provenance,
Cargo.lock and build output are not shipped. See [CONTENT](../CONTENT.md) for the
content setup. No standalone vanilla assets are included; documentation images
are ordinary in-game screenshots.

## Machine scope

The machine's custom two-slot screen is backed by a five-slot hopper menu.
Deposit moves the carried stack into input slot 0. Compatible input processes
into output slot 1 in 40 ticks by default; the server reload table can override
the duration by input item id.

Item identities are load-fixed. During `register`, the sample captures
`RegistrationContext::registry("minecraft:item")` through `Registrar::item_key`.
Each reload validates its new duration table against those captured keys and
publishes a fresh keyed map; ticks never resolve an item name. Unknown item ids
refuse the reload rather than silently adding an unusable duration.

`ItemStack` is an owned, nonempty value. Empty slots use `None`, consumed stacks
use `shrink` or `split`, and growth uses fallible `set_count` after the capacity
check. Merging compares both item identity and component patches, including
numeric damage presence. The game's write boundary owns registry-dependent
validation; the sample does not duplicate that component validator.

It pulls one item from slot 0 of a container directly west through that
container's east face. A pipe directly east sends one output item per tick into
a container east of the pipe through its west face. Other orientations and pipe
networks are not supported. Both machine slots are exposed on every face;
slot 0 accepts input and slot 1 allows output.

Registration declares the `machine` block-entity type for the machine block,
retaining save id `machine_sample:machine`, and attaches its scoped Part through
the returned key. Empty initial NBT uses the type's default. Each scoped Part
uses `#[mod_api::horizon_part]` to declare subscriptions. The pipe is a plain
block. Inventory transfers reuse `ITEMS`, the tank reuses `FLUIDS`, and the
energy buffer reuses `ENERGY`; there are no parallel stores or transfer engines.

To test a hook or helper end to end, change the trait mod, build the linked game
and exercise it on the intended game thread. This is the recommended sample
route for native mod API work described in
[Horizon #4953](https://github.com/horizon-rust/horizon/issues/4953).
