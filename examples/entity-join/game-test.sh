#!/usr/bin/env bash
set -euo pipefail
# Arguments: horizon-ctl, built client, prepared resources, absolute content path.
if [ "$#" -ne 4 ]; then
    echo "usage: $0 HORIZON_CTL CLIENT RESOURCES CONTENT" >&2
    exit 2
fi
ctl=$1 client=$2 resources=$3 content=$4
run=$(mktemp -d "${TMPDIR:-/tmp}/entity-join-test.XXXXXX")
export HORIZON_HOME="$run/game" HORIZON_LAUNCHER_HOME="$run/launcher"
export XDG_DATA_HOME="$run/data" XDG_CONFIG_HOME="$run/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
control() { "$ctl" --socket "$run/control.sock" "$@"; }
trap 'control quit >/dev/null 2>&1 || true; echo "Game logs: $run/control.log"' EXIT
control launch --game "$client" --world JoinExample --creative --offscreen --wait --resources "$resources" --dev-mod "$content" --game-dir "$run/game"
control command 'tp @s 0 110 4'
control wait 80
control fill -2 109 -2 3 109 6 minecraft:stone
control command 'summon item 0.5 110 0.5 {Item:{id:"minecraft:diamond",count:2},NoGravity:1b}'
control command 'summon experience_orb 1.5 110 0.5 {Value:7,NoGravity:1b}'
control wait 40
# Require the hook's own messages, not merely successful summon commands.
grep -E 'entity-join-example .*kind=(minecraft:)?item([[:space:]]|$)' "$run/control.log"
grep -E 'entity-join-example .*kind=(minecraft:)?experience_orb([[:space:]]|$)' "$run/control.log"
echo "Both entity joins reached the hook."
