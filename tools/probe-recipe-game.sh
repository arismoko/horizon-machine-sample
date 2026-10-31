#!/bin/bash
set -eu
BASE="$HOME/.cache/horizon-pr-reviewer"
HOME_DIR="$(mktemp -d "$BASE/home-recipe-XXXXXX")"
export HORIZON_HOME="$HOME_DIR/game" HORIZON_LAUNCHER_HOME="$HOME_DIR/launcher"
export XDG_DATA_HOME="$HOME_DIR/data" XDG_CONFIG_HOME="$HOME_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
CTL="$HOME/.cache/horizon-sample-mod/target/fast/horizon-ctl"
SOCK="$HOME_DIR/control.sock"
control() { "$CTL" --socket "$SOCK" "$@"; }
trap 'control quit || true' EXIT
control launch --game "$BASE/target-recipe/fast/horizon-client" --world Probe --creative --offscreen --wait --resources "$HOME/.cache/horizon-upstream-build2-input/local/horizon-client/26.3" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
control command 'tp @s 0 110 4'
control wait 40
control fill -2 109 -2 4 109 6 minecraft:stone
control command 'tp @s 0 110 4'
control setblock 2 110 0 minecraft:chest
control setblock 1 110 0 machine_sample:pipe
control setblock 0 110 0 machine_sample:machine
control container 0 110 0 minecraft:cobblestone 4
control interact 0 110 0
control wait 12
mkdir -p evidence
control screenshot "$PWD/evidence/recipe-progress.png"
echo 'MACHINE DURING PROCESSING'
control ui
control close
control wait 220
echo 'CHEST AFTER PROCESSING'
control container 2 110 0
echo 'MACHINE AFTER PROCESSING'
control container 0 110 0
