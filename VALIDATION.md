# Validation

The sample is being ported to a trait mod at fork Horizon
`c64f46ac4fe95b02d5a14bacf074b1991d187ae1`.

| Check | Result |
|---|---|
| `tools/test_stage.py` | Pass: committed source export and trait bundle staging; source checkout unchanged; repeated export deterministic. |
| `tools/horizon.py check` | Pass: trait crate compiles against the pin. This does **not** link a game executable. |
| `cargo fmt --all --check`; workspace clippy and tests | Pass; clippy warnings denied, 3 runtime tests pass. |
| `tools/test_package.py` | Pass: deterministic package and native bundle mapping. |
| Launcher `native-build --profile dev` | Pass: trait executable built and linked at the pin. |
| Integrated-server game | Pass: four cobblestone from the west chest became four stone in the east chest through the machine and pipe; screen opened and drew. Deposit moved four carried cobblestone into the input, and four more stone reached the east chest. |

The in-game test used the launcher's linked client, the dev profile and the
integrated server thread. The machine pulled from the west neighbor's east face
and pushed to the east neighbor's west face through `ITEMS`. This is not a
dedicated-server test. No player data or Minecraft assets belong in this
repository.
