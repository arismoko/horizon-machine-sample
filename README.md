# Horizon machine sample

This native trait mod demonstrates a small machine, its screen, a recipe, and a
fixed pipe transfer in Horizon. It also provides an end-to-end test for the
native mod API.

![A machine connected to a pipe and chest in the world](docs/images/world.png)

The machine has a custom two-slot screen backed by a five-slot hopper menu.
The Deposit button moves the carried stack into input slot 0; compatible items
are processed in 40 ticks into output slot 1. It pulls one item from slot 0 of a
container directly west, through that container's east face. A pipe directly
east sends one output item per tick into a container east of the pipe, through
its west face. Other orientations and pipe networks are not supported. The
machine exposes both slots on every face; slot 0 accepts input and slot 1 allows
output.

## Build and test

The sample pins fork Horizon commit `5eb3fd87ccb4ec423920c799be21ef8e18d5ca88`.
The exact Horizon and Mixins revisions are in [horizon-mod.toml](horizon-mod.toml).
See [VALIDATION.md](VALIDATION.md) for the scope of the current checks.
Native mods run code with the player's rights; only build code you trust. The
launcher builds the sample from the source the mod pins and links its generated
entry into the game. With a scratch launcher instance and an installed base version:

```sh
horizon-launcher native-build "$PWD" --instance INSTANCE --trust-native-code --profile dev
```

Play the resulting build with this content mod enabled in a fresh world. See
[CONTENT.md](CONTENT.md) for packaging and [VALIDATION.md](VALIDATION.md) for
what has and has not been verified. For local source and trait-crate checks:

```sh
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
cargo +1.99.1 fmt --all --check
cargo +1.99.1 clippy --workspace --all-targets --locked --offline -- -D warnings
cargo +1.99.1 test --workspace --locked --offline
python3 tools/test_stage.py --horizon /path/to/horizon
python3 tools/horizon.py check --horizon /path/to/horizon --target-dir "$PWD/target/horizon-cargo"
```

The stage test exports the pinned committed source, verifies shared dependency
versions against its lockfile, and stages the flat package using the pinned
game's shared package writer; the check compiles
the trait crate, **not** a linked executable. Both leave the
original checkout unchanged. The launcher's native build is necessary to verify
linking and gameplay. `--offline` needs the dependencies in Cargo's cache.

## Package and API

Install `cargo-horizon-mod` from the matching Horizon fork build. The project
and archive have the same flat layout:

```text
horizon-mod.toml
Cargo.toml
src/lib.rs
src/machine.rs
assets/machine_sample/
data/machine_sample/
```

`horizon-mod.toml` is the sole mod manifest: it owns the id and version and
its `[native]` section owns the game and exact source pins. Cargo's package
name and version are not mod identity. `src/lib.rs` marks its `Hooks`
implementation with `#[mod_api::horizon_mod]`, which exports the entry and
registers the machine's `BlockEntityPart`. Registration declares the unqualified
`machine` type for the existing machine block and attaches its Part through the
returned type key. The declaration retains save id `machine_sample:machine`;
empty initial NBT uses its default, with no factory override. The pipe remains
a plain block. Each scoped Part implementation uses
`#[mod_api::horizon_part]` to declare its hook subscriptions at compile time.
The machine uses `ITEMS` to reach neighboring inventories and
scoped screen and menu hooks for its UI. There are no raw injections or
handwritten factories. The launcher generates its own
`horizon-mixin.toml` in its build area; legacy `native/`, `rust/` and
`horizon-native.json` inputs are not supported. Mods don't ship a toolchain
file; Horizon's pinned toolchain builds them. The local helper and CI use Rust
1.99.1, matching the pinned Horizon launcher's toolchain.

Run `python3 tools/package.py --horizon /path/to/horizon` to create
`target/horizon-mod/machine_sample.hmod` with the pinned game's shared writer,
not a second Python ZIP implementation. `python3 tools/test_package.py` checks
the canonical archive, determinism and flat staging.

`tools/horizon.py` exports committed sources into ignored `target/` trees,
verifies source hashes and leaves the original checkout and game resources
unchanged. `compatibility.json` is developer provenance for those exports,
not a packaged or runtime manifest. Its hashes and revision, and the standalone
Cargo dependency/patch pins, must agree with `horizon-mod.toml`.
The development `Cargo.lock` is seeded from that Horizon revision's committed
lockfile before the standalone crate is resolved; it is not packaged. The
launcher uses the game's lockfile when staging the sole native crate.
No game assets are included.

To test a hook or helper end to end, change the trait mod, build the linked game
with the launcher and exercise the behavior on the intended game thread. This
is the recommended mod test route for hook work in
[Horizon #4953](https://github.com/horizon-rust/horizon/issues/4953).
