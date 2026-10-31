# Horizon machine sample

This native hook bundle is a small example for Horizon commit
`36f4bc00d6fc644f1ad118ddd926a997a034cb48`. Install the matching
`machine_sample` content mod separately. The machine is designed to process
`machine_sample:processing` recipes in 40 ticks, with an input slot, an output
slot and a progress bar. **Processing does not work on the normal server thread
at this pin:** the recipe helpers return no recipes when the server has no
local player. Horizon issue #5119 tracks this; draft fix #5120 is not merged.
The screen opens, but its progress bar stays empty until that engine gap is fixed.

To transfer output, place one `machine_sample:pipe` immediately east of the
machine and a container immediately east of the pipe. The machine attempts to
insert one item into the container's west face each tick. This transfer was
tested with a manually loaded output stack. Other orientations and pipe networks
are not supported.

The bundle runs native code with the player's rights. The staging tools export
committed Horizon sources and apply the mixins without changing the checkout.
Set `HORIZON_MIXINS` to the pinned Mixins checkout in `compatibility.json`.
For example:

```sh
cargo test --workspace --locked --offline
python3 tools/horizon.py stage --horizon /path/to/horizon
python3 tools/horizon.py check --horizon /path/to/horizon --target-dir /path/to/cargo-target
```

Use prepared resources from the matching game version. No game assets are
included in this repository. See `AGENTS.md` and `tools/` for staging and
source audit tools.

The legacy `tools/test_template.py` fixture targets the original break-block
example. It does not validate this bundle. Use `tools/test_stage.py` against
the pinned Horizon source instead.
