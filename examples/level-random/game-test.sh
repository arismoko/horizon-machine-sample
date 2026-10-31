#!/bin/bash
set -eu
: "${CTL:?Set CTL to horizon-ctl}"
: "${CLIENT:?Set CLIENT to the built horizon-client}"
: "${RESOURCES:?Set RESOURCES to prepared 26.3 resources}"
BASE="${TMPDIR:-${XDG_CACHE_HOME:-$HOME/.cache}/horizon-sample-mod}"
mkdir -p "$BASE"
HOME_DIR="$(mktemp -d "$BASE/level-random-game-XXXXXX")"
echo "Game test files: $HOME_DIR"
export HORIZON_HOME="$HOME_DIR/game" HORIZON_LAUNCHER_HOME="$HOME_DIR/launcher"
export XDG_DATA_HOME="$HOME_DIR/data" XDG_CONFIG_HOME="$HOME_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
SOCK="$HOME_DIR/control.sock"
control() { "$CTL" --socket "$SOCK" "$@"; }
trap 'control quit || true' EXIT
control launch --game "$CLIENT" --world RandomTest --creative --offscreen --wait --resources "$RESOURCES" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
control command 'tp @s 0 110 4'
control wait 80
control fill -2 109 -2 3 109 6 minecraft:stone
control setblock 0 110 0 machine_sample:machine
control wait 80
grep -E 'PR5146 random overworld=Some\(-?[0-9]+\) invalid_none=true' "$HOME_DIR/control.log"
control quit
trap - EXIT
