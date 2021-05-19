#!/usr/bin/env bash

# record the selected area as GIF with displaying pressed keys
#
# usage: rec.sh <name of the recording>
# press right control key to stop recording
#
# depends on: [git, slop, screenkey, menyoki]

geometry=$(slop -n -f '%g')
path=$(git rev-parse --show-toplevel)

screenkey \
    --key-mode keysyms \
    -t 1.5 -s small \
    -g "$geometry" -p bottom \
    --opacity 0.7 --ignore Control_R &
screenkey_pid=$!

menyoki record \
    --action-keys RControl \
    --root --size "$geometry" \
    gif --gifski \
    save -e "$path/demo/gpg-tui-${1:-demo}"

kill $screenkey_pid
