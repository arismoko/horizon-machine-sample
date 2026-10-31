#!/bin/bash
set -eu
BASE="$HOME/.cache/horizon-pr-reviewer"
HOME_DIR="$(mktemp -d "$BASE/home-5167-XXXXXX")"
export HORIZON_HOME="$HOME_DIR/game" HORIZON_LAUNCHER_HOME="$HOME_DIR/launcher"
export XDG_DATA_HOME="$HOME_DIR/data" XDG_CONFIG_HOME="$HOME_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
CTL="$HOME/.cache/horizon-sample-mod/target/fast/horizon-ctl"
SOCK="$HOME_DIR/control.sock"
control() { "$CTL" --socket "$SOCK" "$@"; }
trap 'control quit || true; grep -E "PR5167|panicked|thread.*panic" "$HOME_DIR/control.log" || true' EXIT
control launch --game "$BASE/target-5167/fast/horizon-client" --world Probe --creative --offscreen --wait --resources "$HOME/.cache/horizon-upstream-build2-input/local/horizon-client/26.3" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
control command 'tp @s 0 110 4'
control wait 80
control fill -2 109 -2 3 109 6 minecraft:stone
control command 'summon item 0.5 110 0.5 {Item:{id:"minecraft:diamond",count:2},NoGravity:1b}'
control command 'summon experience_orb 1.5 110 0.5 {Value:7,NoGravity:1b}'
control wait 40
control look-at 0.5 110 0.5
mkdir -p evidence
control screenshot "$PWD/evidence/pr5167.png"
control entities 16 minecraft:item
control entities 16 minecraft:experience_orb
control command 'kill @e[type=minecraft:item]'
control command 'tp @s 0 250 0'
control wait 80
control give minecraft:diamond 1
control key q tap || true
sleep 3
control status || true
