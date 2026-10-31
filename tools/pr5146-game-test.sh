#!/bin/bash
set -eu
BASE="$HOME/.cache/horizon-pr-reviewer"
HOME_DIR="$(mktemp -d "$BASE/home-5146-XXXXXX")"
export HORIZON_HOME="$HOME_DIR/game" HORIZON_LAUNCHER_HOME="$HOME_DIR/launcher"
export XDG_DATA_HOME="$HOME_DIR/data" XDG_CONFIG_HOME="$HOME_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
CTL="$HOME/.cache/horizon-sample-mod/target/fast/horizon-ctl"
SOCK="$HOME_DIR/control.sock"
control() { "$CTL" --socket "$SOCK" "$@"; }
trap 'control quit || true' EXIT
control launch --game "$BASE/target-5146/fast/horizon-client" --world Probe --creative --offscreen --wait --resources "$HOME/.cache/horizon-upstream-build2-input/local/horizon-client/26.3" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
control command 'tp @s 0 110 4'
control wait 80
control fill -2 109 -2 3 109 6 minecraft:stone
control command 'tp @s 0 110 4'
control setblock 0 110 0 machine_sample:machine
control wait 80
control look-at-block 0 110 0
mkdir -p evidence
control screenshot "$PWD/evidence/pr5146.png"
grep 'PR5146' "$HOME_DIR/control.log" || true
control quit
trap - EXIT
