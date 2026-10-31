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
control launch --game "$BASE/target-west-input/fast/horizon-client" --world Probe --creative --offscreen --wait --resources "$HOME/.cache/horizon-upstream-build2-input/local/horizon-client/26.3" --dev-mod "$PWD/content" --game-dir "$HOME_DIR/game"
check() {
    local x="$1" slot="$2" item="$3" count="$4"
    control container "$x" 110 0 | python3 -c 'import json,sys
slot,item,count=int(sys.argv[1]),sys.argv[2],int(sys.argv[3])
actual=json.load(sys.stdin)["slots"][slot]
expected=None if item=="empty" else {"item":item,"count":count}
assert actual==expected, f"slot {slot}: expected {expected}, got {actual}"
print(f"slot {slot}: {actual}")' "$slot" "$item" "$count"
}
control command 'tp @s 0 110 5'
control wait 80
control fill -3 109 -2 15 109 7 minecraft:stone
control command 'tp @s 0 110 5'
control setblock -1 110 0 minecraft:chest
control container -1 110 0 minecraft:diamond 3
control setblock 0 110 0 machine_sample:machine
control wait 40
check -1 0 minecraft:diamond 2
check 0 0 minecraft:diamond 1
control wait 80
check -1 0 minecraft:diamond 2
check 0 0 minecraft:diamond 1
control setblock 5 110 0 minecraft:chest
control setblock 6 110 0 machine_sample:machine
control setblock 12 110 0 machine_sample:machine
control wait 40
check 5 0 empty 0
check 6 0 empty 0
check 12 0 empty 0
control look-at-block 0 110 0
control interact 0 110 0
control wait 10
mkdir -p evidence
control screenshot "$PWD/evidence/west-input.png"
control quit
trap - EXIT
