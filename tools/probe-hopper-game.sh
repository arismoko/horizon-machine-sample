#!/bin/bash
set -eu
BASE="$HOME/.cache/horizon-pr-reviewer"
HOME_DIR="$(mktemp -d "$BASE/home-hopper-XXXXXX")"
export HORIZON_HOME="$HOME_DIR/game" HORIZON_LAUNCHER_HOME="$HOME_DIR/launcher"
export XDG_DATA_HOME="$HOME_DIR/data" XDG_CONFIG_HOME="$HOME_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
CTL="$HOME/.cache/horizon-sample-mod/target/fast/horizon-ctl"
SOCK="$HOME_DIR/control.sock"
control() { "$CTL" --socket "$SOCK" "$@"; }
trap 'control quit || true' EXIT
control launch --game "$BASE/target-hopper/fast/horizon-client" --world Probe --creative --offscreen --wait --resources "$HOME/.cache/horizon-upstream-build2-input/local/horizon-client/26.3" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
control command 'tp @s 0 110 4'
control wait 40
control fill -2 109 -2 3 109 6 minecraft:stone
control command 'tp @s 0 110 4'
control setblock 0 110 0 machine_sample:machine
control container 0 110 0 minecraft:diamond 3
control container 0 110 0 minecraft:emerald 2
control wait 20
control interact 0 110 0
echo 'OPEN INITIAL'
control ui
control click-slot 0
control click-slot 32
control click-slot 1
control click-slot 33
echo 'AFTER MACHINE TO INVENTORY'
control ui
control click-slot 32
control click-slot 0
control click-slot 33
control click-slot 1
echo 'AFTER INVENTORY TO MACHINE'
control ui
control close
control wait 10
echo 'SERVER CONTAINER AFTER CLOSE'
control container 0 110 0
control interact 0 110 0
echo 'AFTER REOPEN'
control ui
mkdir -p evidence
control screenshot "$PWD/evidence/hopper-screen.png"
