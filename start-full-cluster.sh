#!/bin/bash
set -e

echo "Starting the cluster..."
# node 1
RUST_LOG=INFO ./target/release/raft-log-store --id 1 --http-addr 0.0.0.0:9050 2>&1 > ./node-1.log &
# node 2
RUST_LOG=INFO ./target/release/raft-log-store --id 2 --http-addr 0.0.0.0:9051 2>&1 > ./node-2.log &
# node 3
RUST_LOG=INFO ./target/release/raft-log-store --id 3 --http-addr 0.0.0.0:9052 2>&1 > ./node-3.log &

echo "Initializing the cluster..."
curl -v -H "Content-Type: application/json" "http://127.0.0.1:9050/init" -d '{}' | jq

echo "Metrics:"
curl -v "http://127.0.0.1:9050/metrics" | jq

sleep 1

echo "Add node 2 to the cluster..."
curl -v -H "Content-Type: application/json" "http://127.0.0.1:9050/add-learner" -d '[2, "127.0.0.1:9051"]' | jq

sleep 1

echo "Add node 3 to the cluster..."
curl -v -H "Content-Type: application/json" "http://127.0.0.1:9050/add-learner" -d '[3, "127.0.0.1:9052"]' | jq

sleep 1

echo "Update membership..."
curl -v -H "Content-Type: application/json" "http://127.0.0.1:9050/change-membership" -d '[1, 2, 3]' | jq

sleep 1

echo "Metrics:"
curl -v "http://127.0.0.1:9050/metrics" | jq

echo "Cluster started."