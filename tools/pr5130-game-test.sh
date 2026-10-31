#!/bin/bash
set -eu
BASE="$HOME/.cache/horizon-pr-reviewer"
HOME_DIR="$(mktemp -d "$BASE/home-5130-XXXXXX")"
export HORIZON_HOME="$HOME_DIR/game" HORIZON_LAUNCHER_HOME="$HOME_DIR/launcher"
export XDG_DATA_HOME="$HOME_DIR/data" XDG_CONFIG_HOME="$HOME_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
CTL="$HOME/.cache/horizon-sample-mod/target/fast/horizon-ctl"
SOCK="$HOME_DIR/control.sock"
control() { "$CTL" --socket "$SOCK" "$@"; }
trap 'control quit || true' EXIT
control launch --game "$BASE/target-5130/fast/horizon-client" --world Probe --creative --offscreen --wait --resources "$HOME/.cache/horizon-upstream-build2-input/local/horizon-client/26.3" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
control command 'tp @s 0 110 5'
control wait 80
control fill -3 109 -2 6 109 7 minecraft:stone
control setblock -1 110 0 minecraft:chest
control setblock 2 110 0 minecraft:chest
control setblock 4 110 0 minecraft:chest
control container -1 110 0 minecraft:cobblestone 3
control container 2 110 0 minecraft:diamond 3
control setblock 0 110 0 machine_sample:machine
control wait 100
echo 'SOURCE:'
control container -1 110 0
echo 'MACHINE:'
control container 0 110 0
echo 'MOVE SOURCE:'
control container 2 110 0
echo 'MOVE TARGET:'
control container 4 110 0
control command 'tp @s 3 110 3'
control wait 20
control look-at-block 4 110 0
control interact 4 110 0
control wait 20
mkdir -p evidence
control screenshot "$PWD/evidence/pr5130.png"
control logs --lines 300
