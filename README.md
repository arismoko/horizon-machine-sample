# Horizon machine sample

A small native Horizon mod with a working machine, a screen and a pipe. It also
serves as a tour of the mod API: each example is a small module you can read,
change and try in a fresh world.

![Machine between an input chest and an output pipe and chest](docs/images/tour-world.png)

## What's inside

| Example | What it demonstrates | In the game |
| --- | --- | --- |
| [Machine](src/machine.rs) | Pull an item from the west, process it, and send the result east through a pipe. | <img src="docs/images/tour-screen.png" width="220" alt="Machine screen processing cobblestone, with overlay and reloadable label"> |
| [Services](src/tour/services.rs) | An operator announcement command with typed sources and source-routed feedback. | Chat messages |
| [Fluid tank](src/tour/fluids.rs) | A saved 4,000 mB tank; fill and drain it using a held bucket and transactions. | <img src="docs/images/tour-fluid.png" width="280" alt="Successful 1000 mB tank fill with the emptied bucket in hand"> |
| [Energy](src/tour/energy.rs) | Recover 25 energy per recipe into a saved buffer exposed on every face. | Saved block-entity data |
| [Client](src/tour/client.rs) | A remappable inspection key, camera changes, local sound and a screen overlay. | <img src="docs/images/tour-keybinding.png" width="280" alt="Inspect machine view keybinding set to I"> |
| [Reload](src/tour/reload.rs) | Read processing times from `data/` and a screen label from `assets/`, with pack overrides. | Label above the machine screen |
| [Tags and gamerules](src/tour/policy.rs) | A reloadable input tag and a gamerule that pauses automatic intake. | Recipe membership and intake policy |
| [Attachments](src/tour/attachments.rs) | Inspect an entity with an empty main hand to record a persistent visit count. | <img src="docs/images/tour-attachment.png" width="280" alt="Cow inspection counter displayed in the player's action bar"> |

The default recipe turns cobblestone into stone in 40 ticks. The machine has two
visible slots; its Deposit button accepts the carried stack. This is a fixed
west-to-east transfer example, not a pipe network.

[Try each example](VALIDATION.md#feature-tour-triggers) for the commands, gestures
and expected results.

## Build and run

Sign in to a Minecraft: Java Edition account in the fork's launcher and install
a build matching the sample's [pinned source](horizon-mod.toml).
Native mods run with your rights: read the
source before trusting it. In the commands below, replace `PINNED_BUILD` with that
installed build's version; use a fresh world and enable the sample's content.

```sh
git clone https://github.com/arismoko/horizon-machine-sample.git
cd horizon-machine-sample
horizon-launcher instance create "Machine Sample"
horizon-launcher native-build "$PWD" --instance "Machine Sample" --version PINNED_BUILD --trust-native-code --profile developer
horizon-launcher play --instance "Machine Sample"
```

[Development and packaging](docs/DEVELOPMENT.md) covers exact revisions, the flat
package layout, source checks and common build prerequisites.

## How it's tested

- CI checks formatting, clippy and standalone unit tests.
- The staging test checks the pinned source export, canonical package writer,
  deterministic output and that the original checkout stays unchanged.
- The pinned check compiles the trait crate. It does **not** link or run the game.
- These pictures come from an actual linked Developer build on software Vulkan.
  Processing and pipe output, tank transfers, saved machine data, and an entity's
  inspection counter across save/reopen were exercised. They are not proof of
  every API path; keyboard camera/sound and pack overrides remain unverified.

[Validation](VALIDATION.md) separates automated checks from gameplay checks and
lists the triggers for every module. [Contributing](CONTRIBUTING.md) has the
commands to run before a code change.
