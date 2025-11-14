#!/bin/bash
# Start one slow request, then 3 fast ones
echo "Starting slow request... (4-5s estimated)"
curl -s --limit-rate 4k http://127.0.0.1:8080/Cargo.lock > /dev/null && echo -n "Slow request done at " && date +%X &

sleep 1  # Give slow request a head start

echo "Starting fast request #1..."
curl -s http://127.0.0.1:8080/README.md > /dev/null && echo -n "Fast request #1 done at " && date +%X &
echo "Starting fast request #2..."
curl -s http://127.0.0.1:8080/index.html > /dev/null && echo -n "Fast request #2 done at " && date +%X &
echo "Starting fast request #3..."
curl -s http://127.0.0.1:8080/Cargo.toml > /dev/null && echo -n "Fast request #3 done at " && date +%X &

wait
echo "All done!"
