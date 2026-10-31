# Horizon machine sample

This native mod shows a small machine, its screen, a recipe, and a fixed pipe
transfer in Horizon. It also provides a way to test native hooks and helpers
from a mod through to a running game.

![A machine connected to a pipe and chest in the world](docs/images/world.png)

## What's in it

- A machine with input and output slots and a progress bar. Its
  `machine_sample:processing` recipe is designed to finish in 40 ticks.
- One pipe segment that moves output into a container. Place the pipe directly
  east of the machine and the container directly east of the pipe. The machine
  attempts to insert one item per tick into the container's west face. Other
  orientations and pipe networks are not supported.
- A native bundle and a separate `machine_sample` content mod. The bundle ID
  and module name are also `machine_sample`.

![Machine screen showing cobblestone and a partly filled progress bar](docs/images/machine-screen.png)

*Machine screen from a build with Horizon PR #5120 applied, not the pinned build.*

## Try it

The sample pins Horizon commit `979aa64e5199525f7318e7f8be1d4766e76f8c7e`.
The exact Horizon and Mixins revisions are in [compatibility.json](compatibility.json).
The native bundle runs code with the player's rights. Only use native mods you
trust. Set `HORIZON_MIXINS` to a checkout at the pinned Mixins revision.

```sh
cargo test --workspace --locked --offline
python3 tools/horizon.py stage --horizon /path/to/horizon
python3 tools/horizon.py check --horizon /path/to/horizon --target-dir /path/to/cargo-target
python3 tools/horizon.py build --horizon /path/to/horizon --target-dir /path/to/cargo-target
```

The build prints the `horizon-client` path. Run it with your own prepared 26.3
resources and this repository's content folder:

```sh
/path/to/cargo-target/fast/horizon-client --resources /path/to/prepared/26.3 \
  --game-dir /path/to/a/fresh/game --dev-mod "$PWD/content" --world Sample --creative
```

The game loads the content folder directly for this run. For a packaged content
mod and installation steps, see [content/README.md](content/README.md).

## Test a hook or helper end to end

Change the bundle under `mixins/machine_sample/` to call the hook or helper you
want to test. Run the standalone tests, stage and build the pinned client, then
launch it with the content folder as shown above. Exercise the behavior in a
fresh world and check the result on the intended game thread. This is the
recommended mod test route for hook work in
[Horizon #4953](https://github.com/horizon-rust/horizon/issues/4953).
The staging tools export committed Horizon sources; they do not change the
original checkout.

## Recipe status

At the pinned revision, processing does **not** work on the normal server
thread. The recipe helpers return no recipes when the server has no local
player. [Horizon #5119](https://github.com/horizon-rust/horizon/issues/5119)
tracks the gap; draft [PR #5120](https://github.com/horizon-rust/horizon/pull/5120)
is not merged. The screen opens, but progress stays empty at this pin.

With PR #5120 applied, a test at commit `3b77664bd` processed four cobblestone
into four stone and transferred them to a chest. The pinned revision does not
include that fix. The pipe also worked at the pin with a manually loaded output
stack.

## Notes

The legacy `tools/test_template.py` fixture targets the original break-block
example; it does not validate this bundle. Use `tools/test_stage.py` against
the pinned Horizon source instead. See [CONTRIBUTING.md](CONTRIBUTING.md) and
`tools/` for validation and source audit details. The repository does not
include game assets.
