#!/bin/bash
set -e
echo "Running full integration scenario..."

# Lancer un worker mock en arrière-plan
cargo run -p gridflow-scheduler --example mock_worker &
WORKER_PID=$!

sleep 3

# Exemple d'appel CLI (à adapter)
cargo run -p gridflow-scheduler --bin gridflow -- submit --task-type compute --input-file data.json || true

kill ${WORKER_PID} || true
