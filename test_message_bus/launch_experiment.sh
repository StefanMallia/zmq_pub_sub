#!/bin/bash
#subscribers start before publishers

CARGO_RUN_COMMAND="cargo run --release"
CARGO_BUILD_COMMAND="cargo build --release"
$CARGO_BUILD_COMMAND --manifest-path=$PWD/message_bus/Cargo.toml
$CARGO_BUILD_COMMAND --manifest-path=$PWD/subscriber_app/Cargo.toml
$CARGO_BUILD_COMMAND --manifest-path=$PWD/publisher_app/Cargo.toml

#gnome-terminal --working-directory=$PWD/message_bus -- bash -c "$CARGO_RUN_COMMAND; bash" 

#gnome-terminal --working-directory=$PWD/subscriber_app -- bash -c "$CARGO_RUN_COMMAND; bash" 
#gnome-terminal --working-directory=$PWD/subscriber_app -- bash -c "$CARGO_RUN_COMMAND; bash" 
sleep 2

LAUNCH_MANY_PUBLISHERS="$CARGO_RUN_COMMAND 1"
#  & $CARGO_RUN_COMMAND 2 \
#  & $CARGO_RUN_COMMAND 3 \
#  & $CARGO_RUN_COMMAND 4 \
#  & $CARGO_RUN_COMMAND 5 \
#  & $CARGO_RUN_COMMAND 6 \
#  & $CARGO_RUN_COMMAND 7 \
#  & $CARGO_RUN_COMMAND 8 \
#  & $CARGO_RUN_COMMAND 9 \
#  & $CARGO_RUN_COMMAND 10 \
#  & $CARGO_RUN_COMMAND 11 \
#  & $CARGO_RUN_COMMAND 12 \
#  & $CARGO_RUN_COMMAND 13 \
#  & $CARGO_RUN_COMMAND 14 \
#  & $CARGO_RUN_COMMAND 15 \
#  & $CARGO_RUN_COMMAND 16 \
#  & $CARGO_RUN_COMMAND 17 \
#  & $CARGO_RUN_COMMAND 18 \
#  & $CARGO_RUN_COMMAND 19 \
#  & $CARGO_RUN_COMMAND 20"


tmux new-session -d\
    "cd $PWD/message_bus && $CARGO_RUN_COMMAND; bash" \;\
    split-window -h "cd $PWD/subscriber_app && $CARGO_RUN_COMMAND; bash" \;\
    select-pane -t 1 \;\
    split-window -v "cd $PWD/publisher_app && ($LAUNCH_MANY_PUBLISHERS); bash" \;\
    attach\;
