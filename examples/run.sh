#!/usr/bin/env bash
set -euo pipefail
# Build minimal PoCs
gcc -O2 -o poCs/cache/victim_sim poCs/cache/victim_sim.c || { echo "gcc failed"; exit 1; }
gcc -O2 -o poCs/cache/flush_reload_attacker poCs/cache/flush_reload_attacker.c -march=native || { echo "gcc failed"; exit 1; }
# Run victim in background
./poCs/cache/victim_sim &
VICTIM_PID=$!
sleep 0.1
# Run attacker
./poCs/cache/flush_reload_attacker
# Cleanup
kill ${VICTIM_PID} || true
