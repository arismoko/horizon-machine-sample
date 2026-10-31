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
check_slot() {
    local slot="$1" item="$2" count="$3"
    control wait 3 >/dev/null
    control ui | python3 -c 'import json,sys
slot, item, count = sys.argv[1], sys.argv[2], int(sys.argv[3])
ui = json.load(sys.stdin)
w = next(w for w in ui["widgets"] if w["id"] == "slot-" + slot)
expected = None if item == "empty" else {"item": item, "count": count}
assert w["stack"] == expected, f"slot {slot}: expected {expected}, got {w[chr(115)+chr(116)+chr(97)+chr(99)+chr(107)]}"
print(f"slot {slot}: {expected}")' "$slot" "$item" "$count"
}
check_slot 300 minecraft:diamond 3
check_slot 301 minecraft:emerald 2
control click-slot 300
check_slot 300 empty 0
control click-slot 0
check_slot 0 minecraft:diamond 3
control click-slot 301
check_slot 301 empty 0
control click-slot 1
check_slot 1 minecraft:emerald 2
control click-slot 0
check_slot 0 empty 0
control click-slot 300
check_slot 300 minecraft:diamond 3
control click-slot 1
check_slot 1 empty 0
control click-slot 301
check_slot 301 minecraft:emerald 2
control close
control wait 10
control container 0 110 0
control interact 0 110 0
check_slot 300 minecraft:diamond 3
check_slot 301 minecraft:emerald 2
mkdir -p evidence
control screenshot "$PWD/evidence/hopper-screen.png"
