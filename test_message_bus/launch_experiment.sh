#!/bin/bash
COMMAND="cargo run"
gnome-terminal --working-directory=$PWD/message_bus -- bash -c "$COMMAND; bash" 
gnome-terminal --working-directory=$PWD/publisher_app1 -- \
  bash -c "$COMMAND 1 \
  & $COMMAND 2 \
  & $COMMAND 3 \
  & $COMMAND 4 \
  & $COMMAND 5 \
  & $COMMAND 6 \
  & $COMMAND 7 \
  & $COMMAND 8 \
  & $COMMAND 9 \
  & $COMMAND 10 \
  & $COMMAND 11 \
  & $COMMAND 12 \
  & $COMMAND 13 \
  & $COMMAND 14 \
  & $COMMAND 15 \
  & $COMMAND 16 \
  & $COMMAND 17 \
  & $COMMAND 18; bash"

# publisher_app2 sends bytes
gnome-terminal --working-directory=$PWD/publisher_app2 -- \
  bash -c "$COMMAND 19; bash"


gnome-terminal --working-directory=$PWD/subscriber_app -- bash -c "$COMMAND; bash"
