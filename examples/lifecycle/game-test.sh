#!/usr/bin/env bash
set -eu
: "${CTL:?Set CTL to horizon-ctl}"
: "${CLIENT:?Set CLIENT to the example horizon-client}"
: "${RESOURCES:?Set RESOURCES to prepared 26.3 resources}"
: "${CONTENT:?Set CONTENT to the sample content directory}"
: "${TEST_DIR:?Set TEST_DIR to a new absolute directory}"
mkdir "$TEST_DIR"
export HORIZON_HOME="$TEST_DIR/game" HORIZON_LAUNCHER_HOME="$TEST_DIR/launcher"
export XDG_DATA_HOME="$TEST_DIR/data" XDG_CONFIG_HOME="$TEST_DIR/config"
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1 HORIZON_NO_AUDIO=1
control() { "$CTL" --socket "$TEST_DIR/control.sock" "$@"; }
trap 'control quit || true' EXIT
control launch --game "$CLIENT" --world Lifecycle --creative --offscreen --wait --resources "$RESOURCES" --dev-mod "$CONTENT" --game-dir "$TEST_DIR/game"
control wait 80
control command 'tp @s 800 120 0'
control wait 200
control quit
trap - EXIT
# The client exits asynchronously after the control command.
for _ in $(seq 1 30); do
    if grep -q 'PR5131 logout ' "$TEST_DIR/control.log"; then break; fi
    sleep 1
done
for event in 'chunk load' 'chunk unload' login logout; do
    if ! grep -Fq "PR5131 $event " "$TEST_DIR/control.log"; then
        echo "Missing $event in $TEST_DIR/control.log" >&2
        exit 1
    fi
done
echo "All four lifecycle events appear in $TEST_DIR/control.log"
