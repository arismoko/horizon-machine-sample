#!/bin/bash
set -eu
BASE="$HOME/.cache/horizon-pr-reviewer"
HOME_DIR="$(mktemp -d "$BASE/home-5167-crash-XXXXXX")"
export HORIZON_HOME="$HOME_DIR/game" HORIZON_LAUNCHER_HOME="$HOME_DIR/launcher"
export XDG_DATA_HOME="$HOME_DIR/data" XDG_CONFIG_HOME="$HOME_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1 RUST_BACKTRACE=1
CTL="$HOME/.cache/horizon-sample-mod/target/fast/horizon-ctl"
SOCK="$HOME_DIR/control.sock"
control() { "$CTL" --socket "$SOCK" "$@"; }
trap 'control quit || true; grep -A18 -B3 -E "canceled=true|panicked at|panic" "$HOME_DIR/control.log" || true' EXIT
control launch --game "$BASE/target-5167/fast/horizon-client" --world Probe --creative --offscreen --wait --resources "$HOME/.cache/horizon-upstream-build2-input/local/horizon-client/26.3" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
control command 'gamerule random_tick_speed 0'
control command 'gamerule do_mob_spawning false'
control command 'tp @s 0 250 0'
control fill -3 249 -3 3 249 3 minecraft:stone
control command 'tp @s 0 250 0'
control wait 30
control give minecraft:diamond 1
control command 'kill @e[type=minecraft:item]'
control key q tap || true
sleep 3
control status || true
