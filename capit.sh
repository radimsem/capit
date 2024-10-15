#!/usr/bin/env bash

if [ -z $TMUX ]; then
    echo "tmux session not attached"
    exit 1
fi

hash=$(date +"%T.%N" | sha256sum | cut -b -5)
file="$(pwd)/.capit-$hash"

read -p "Enter how many more lines to edit from history buffer: " count

if [ -z "$count" ]; then
    tmux capture-pane -p >> $file
else
    tmux capture-pane -pS "-$count" >> $file
fi

~/.local/bin/capit-trimmer $file

$EDITOR $file
rm $file
