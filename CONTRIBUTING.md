# Contributing

Contributions are welcome. Please open a pull request and describe the change
and its tests. Submit original code and assets only. Do not copy Mojang, Create,
or other mods' source code or assets.

This sample tracks the exact Horizon and Mixins revisions in
`compatibility.json`. Keep changes compatible with those pins, or explain and
validate any pin update. Read the build and content setup in [README.md](README.md)
before starting.

Run these standalone checks:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
```

For bundle or compatibility changes, also run `python3 tools/test_stage.py` and
`python3 tools/horizon.py check` and `test` against the pinned Horizon source.
Pass `--horizon` and set `HORIZON_MIXINS` as described in the README. The old
`tools/test_template.py` fixture targets the template, not this sample.
Do not commit exported sources, game assets, or build output.

Contributions are licensed under the repository's [MIT license](LICENSE).
