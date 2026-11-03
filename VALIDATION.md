# Validation

The flat project currently targets fork Horizon
`0bb67654f331843dd312327c4562d0ba17ead3df`.

This is a development migration, not a claim that this head can run the mod.
The machine keeps both the side-split Host API and the generated
`#[mod_api::horizon_mod]` entry. The target does not yet export that attribute;
its owning Horizon lane must bring in the generated-entry API. No handwritten
factory or fallback is used.

| Check at this flat pin | Result |
|---|---|
| Committed pin, source hashes and standalone lock | Pass: exact source hashes and offline resolution seeded from the pinned Horizon lock. |
| `tools/test_stage.py` | Pending: uses the pinned game's shared package writer and stages its validated flat input. |
| `tools/horizon.py check` | Pending; expected to require the missing generated-entry API. This does not link a game executable. |
| `cargo fmt --all --check` | Pass. |
| Standalone clippy and tests | Not run: generated-entry API is missing at this target. |
| `tools/test_package.py` | Pending: deterministic canonical archive and flat staging. |
| Launcher linked dev build / in-game run | Not performed for the flat migration. |

The previous nested-layout sample had linked and integrated-server machine
proofs, but those are not exact-head evidence for this flat pin. Dedicated-server
proof is also not claimed. No player data or Minecraft assets belong in this
repository.
