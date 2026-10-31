# Machine sample content

This data-only content mod supplies `machine_sample:machine` and
`machine_sample:pipe`, their block items, and original pixel textures. The
machine copies the vanilla chest block template. The pipe copies the iron
block template. Both blocks use simple cube models.

The `machine_sample:processing` recipe maps one cobblestone to one stone.
It is mod-defined data, not a vanilla crafting recipe. It does nothing without
native code that reads this recipe type and implements machine behavior.
The blocks and textures do not provide processing or pipe transport by themselves.

## Build and install

Install the content tool from the matching Horizon revision, as described in
Horizon `docs/mods.md`. From this directory, run:

```sh
cargo horizon-mod build
cargo horizon-mod check target/horizon-mod/machine_sample.hmod
```

Copy `target/horizon-mod/machine_sample.hmod` to the game's `horizon-mods/` folder.
Enable it in Options > Mods, then restart the game. Install the native sample separately using the root
README if its behavior is needed. Use the same content and native mods on
both client and server. Removing registered blocks from an existing world can
lose world data; test in a fresh world.
