# Validation

The flat project pins fork Horizon `f5b891948c0cd5bce563496c5fc129b89cce9efc`,
the main merge of the flat native-mod layout. The sample retains the side-split
Host API and generated `#[mod_api::horizon_mod]` entry from merged sample #20.
There is no handwritten entry, packaging filter or compatibility fallback.

| Check at this flat pin | Result |
|---|---|
| Committed pin, source hashes and standalone lock | Checked before publishing: exact committed hashes and offline resolution seeded from the pinned Horizon lock. |
| `tools/test_stage.py` | Pending the exact-head paired run; uses the pinned game's shared package writer. |
| `tools/horizon.py check` | Pending the exact-head paired run. This compiles the trait crate, not a linked executable. |
| `cargo fmt --all --check` | Checked before publishing. |
| Standalone clippy and tests | Pending at this pin. |
| `tools/test_package.py` | Pending at this pin. |
| Launcher linked dev build / in-game run | Not performed for this flat pin. |

Mods don't ship a toolchain file; Horizon's pinned toolchain builds them. The
helper and CI explicitly select the pinned launcher's Rust 1.99.1. The development
`Cargo.lock` checks shared versions but is not shipped in the package.

The previous nested-layout sample had linked and integrated-server machine
proofs, but those are not exact-head evidence for this flat pin. Dedicated-server
proof is also not claimed. No player data or Minecraft assets belong in this
repository.
