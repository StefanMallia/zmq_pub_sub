#!/bin/bash
#subscribers start before publishers

CARGO_RUN_COMMAND="cargo run --release"
CARGO_BUILD_COMMAND="cargo build --release"
$CARGO_BUILD_COMMAND --manifest-path=$PWD/message_bus/Cargo.toml
$CARGO_BUILD_COMMAND --manifest-path=$PWD/subscriber_app/Cargo.toml
$CARGO_BUILD_COMMAND --manifest-path=$PWD/publisher_app1/Cargo.toml
$CARGO_BUILD_COMMAND --manifest-path=$PWD/publisher_app2/Cargo.toml

#gnome-terminal --working-directory=$PWD/message_bus -- bash -c "$CARGO_RUN_COMMAND; bash" 

#gnome-terminal --working-directory=$PWD/subscriber_app -- bash -c "$CARGO_RUN_COMMAND; bash" 
#gnome-terminal --working-directory=$PWD/subscriber_app -- bash -c "$CARGO_RUN_COMMAND; bash" 
sleep 2

LAUNCH_MANY_PUBLISHERS="$CARGO_RUN_COMMAND 1 \
  & $CARGO_RUN_COMMAND 2 \
  & $CARGO_RUN_COMMAND 3 \
  & $CARGO_RUN_COMMAND 4 \
  & $CARGO_RUN_COMMAND 5 \
  & $CARGO_RUN_COMMAND 6 \
  & $CARGO_RUN_COMMAND 7 \
  & $CARGO_RUN_COMMAND 8 \
  & $CARGO_RUN_COMMAND 9 \
  & $CARGO_RUN_COMMAND 10 \
  & $CARGO_RUN_COMMAND 11 \
  & $CARGO_RUN_COMMAND 12 \
  & $CARGO_RUN_COMMAND 13 \
  & $CARGO_RUN_COMMAND 14 \
  & $CARGO_RUN_COMMAND 15 \
  & $CARGO_RUN_COMMAND 16 \
  & $CARGO_RUN_COMMAND 17 \
  & $CARGO_RUN_COMMAND 18 \
  & $CARGO_RUN_COMMAND 19 \
  & $CARGO_RUN_COMMAND 20"


LAUNCH_MANY_PUBLISHERS2="$CARGO_RUN_COMMAND 21 \
  & $CARGO_RUN_COMMAND 22 \
  & $CARGO_RUN_COMMAND 23 \
  & $CARGO_RUN_COMMAND 24 \
  & $CARGO_RUN_COMMAND 25 \
  & $CARGO_RUN_COMMAND 26 \
  & $CARGO_RUN_COMMAND 27 \
  & $CARGO_RUN_COMMAND 28 \
  & $CARGO_RUN_COMMAND 29 \
  & $CARGO_RUN_COMMAND 30 \
  & $CARGO_RUN_COMMAND 31 \
  & $CARGO_RUN_COMMAND 32 \
  & $CARGO_RUN_COMMAND 33 \
  & $CARGO_RUN_COMMAND 34 \
  & $CARGO_RUN_COMMAND 35 \
  & $CARGO_RUN_COMMAND 36 \
  & $CARGO_RUN_COMMAND 37 \
  & $CARGO_RUN_COMMAND 38 \
  & $CARGO_RUN_COMMAND 39 \
  & $CARGO_RUN_COMMAND 40"

#tmux new-session -d\
#    'cd $PWD/message_bus && htop' \;\
#    split-window 'htop' \;\
#    split-window 'cd $PWD/publisher_app2 && htop' \;\
#    attach\;
#

tmux new-session -d\
    "cd $PWD/message_bus && $CARGO_RUN_COMMAND; bash" \;\
    split-window -h "cd $PWD/subscriber_app && $CARGO_RUN_COMMAND; bash" \;\
    select-pane -t 1 \;\
    split-window -v "cd $PWD/publisher_app1 && ($LAUNCH_MANY_PUBLISHERS); bash" \;\
    select-pane -t 0 \;\
    split-window -v "cd $PWD/publisher_app2 && ($LAUNCH_MANY_PUBLISHERS2); bash" \;\
    attach\;
#
#    
#    split-window 'cd $PWD/publisher_app1 && $LAUNCH_MANY_PUBLISHERS' \;\
