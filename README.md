# Horizon machine sample

This native hook bundle is a small example for Horizon commit
`36f4bc00d6fc644f1ad118ddd926a997a034cb48`. Install the matching
`machine_sample` content mod separately. The machine accepts the
`machine_sample:processing` recipe type, including the sample cobblestone to
stone recipe. It has an input slot, an output slot and a 40-tick progress bar.

To transfer output, place one `machine_sample:pipe` immediately east of the
machine and a container immediately east of the pipe. The machine attempts to
insert one item into the container's west face each tick. Other orientations
and pipe networks are not supported.

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
