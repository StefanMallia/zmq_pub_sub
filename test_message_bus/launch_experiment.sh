#!/bin/bash
COMMAND="cargo run"
gnome-terminal --working-directory=$PWD/message_bus -e "$COMMAND" 
gnome-terminal --working-directory=$PWD/publisher_app -e "$COMMAND 1"
gnome-terminal --working-directory=$PWD/publisher_app -e "$COMMAND 2"
gnome-terminal --working-directory=$PWD/publisher_app -e "$COMMAND 3"
gnome-terminal --working-directory=$PWD/publisher_app -e "$COMMAND 4"
gnome-terminal --working-directory=$PWD/publisher_app -e "$COMMAND 5"
gnome-terminal --working-directory=$PWD/subscriber_app -e "$COMMAND"
