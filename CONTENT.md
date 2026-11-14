# Machine sample content

The top-level content in this mod supplies `machine_sample:machine` and
`machine_sample:pipe`, their block items, and original pixel textures. The
machine copies the vanilla chest block template. The pipe copies the iron
block template. Both blocks use simple cube models.

The `machine_sample:stone_processing` recipe maps one cobblestone to one stone.
It is mod-defined data, not a vanilla crafting recipe. Its file `type` is the
serializer `machine_sample:processing_json`, which the mod registers for the
recipe type `machine_sample:processing`. It does nothing without native code
that reads this recipe type and implements machine behavior.
The blocks and textures do not provide processing or pipe transport by themselves.

## Build and install

Install the content tool from the matching Horizon revision, as described in
Horizon `docs/mods.md`. From this directory, run:

```sh
cargo horizon-mod build
cargo horizon-mod check target/horizon-mod/machine_sample.hmod
```

Install `target/horizon-mod/machine_sample.hmod` through the launcher's Mods page and build its native trait mod. The machine registers a `BlockEntityPart` and uses `ITEMS` and screen hooks; it has no raw injections. Raw mixins are a rare escape hatch, not the recommended route.
This single package contains both content and native behavior. Use the same mod on both client and server. Removing registered blocks from an existing world can
lose world data; test in a fresh world.
