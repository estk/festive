#!/bin/sh

mkfifo pipe
cargo test -- --nocapture > pipe &
dupes=$(cat pipe | grep pid= | sed -E 's/.*pid\=(\d*)/pid=\1/' | uniq -d | wc -c)
rm pipe
if [ $dupes == "0" ]; then
    exit 0;
fi
exit 1