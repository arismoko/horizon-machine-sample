# Experimental native telemetry

This draft tries the prebuilt-game function replacement route. It is not needed
for machine gameplay and does not replace the machine's Part implementation.

- `tick_pass`: a HEAD mixin announces the first block-entity pass.
- `machines`: a published-loop slot counts mod block-entity ticks and announces
  the thousandth. It observes all mod block entities, not exclusively machines.
- `telemetry`: a support module prints autosave countdown telemetry.
- `autosave`: a GET_MUT bridge exposes the private countdown to the telemetry
  helper. The helper only reads it. Run `tick rate 13` to trigger the message.

The source and SDK pin is `13e3b39f7fe0cfeedc85a7aefb30a0e5e270b539`.
The tested loop-fix launcher is built at
`2a7349138a905a41030bce8443be46aabb0a6816` (PR #3467); a source pin alone does
not update the launcher. The normal native launcher build compiles these helpers
into a separate patch library against the prebuilt game, not into the mod's
Cargo crate. `cargo test` alone does not validate these helpers.

These rows are unstable, native-only and tied to the exact game SDK. Use hooks
and Parts for portable behaviour. Do not copy the scene runner's manifest
construction: it appends a second `[native]` table to this sample.
