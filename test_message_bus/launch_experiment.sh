#!/bin/bash
COMMAND="cargo run --release"
gnome-terminal --working-directory=$PWD/message_bus -e "$COMMAND"
gnome-terminal --working-directory=$PWD/publisher -e "$COMMAND 1"
gnome-terminal --working-directory=$PWD/publisher -e "$COMMAND 2"
gnome-terminal --working-directory=$PWD/publisher -e "$COMMAND 3"
gnome-terminal --working-directory=$PWD/publisher -e "$COMMAND 4"
gnome-terminal --working-directory=$PWD/publisher -e "$COMMAND 5"
