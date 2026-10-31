# Validation

Checked on Linux with Rust 1.95.0 against the exact revisions in `compatibility.json`.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | Pass |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Pass |
| `cargo test --workspace --locked --offline` | Pass |
| `python3 tools/test_template.py` | Fails: the legacy fixture lacks the machine bundle's `native_block_entity` target. It does not test this bundle. |
| `python3 tools/test_stage.py --horizon /path/to/horizon` | Pass; deterministic staging and source audit |
| `python3 tools/horizon.py check --horizon /path/to/horizon --target-dir /path/to/cargo-target` | Pass |
| `python3 tools/horizon.py test --horizon /path/to/horizon --target-dir /path/to/cargo-target` | Pass; embedded runtime test |

The native checks reused cached dependencies. They do not validate the screen
in game. No game was launched for this branch. Follow the manual screen check
in README.md with prepared 26.3 resources. The audit covers the declared AST
changes, not compatibility with other revisions or third-party mods.
