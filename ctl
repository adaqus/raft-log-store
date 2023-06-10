#!/bin/bash
set -e

export SERVICE="raft-log-store"

# allow both - and _
readonly SUBCOMMAND="${1//-/_}"
# too few arguments means empty SUBCOMMAND, which triggers an error later
shift || true

readonly USER_ID="$(id -u)"
export USER_ID
readonly GROUP_ID="$(id -g)"
export GROUP_ID

RUST_BINARY_PATH="target/debug/raft-log-store"
export RUST_BINARY_PATH
RUST_LOG="DEBUG"
export RUST_LOG

compile() {
    local ARGS="$@"
    docker compose run -T -e "RUST_LOG=$RUST_LOG" compiler cargo build ${ARGS}
}

recompile() {
    compile
    echo "Killing the $SERVICE..."
    docker compose kill "$SERVICE"

    echo "Removing killed $SERVICE..."
    docker compose rm -f "$SERVICE"

    echo "Starting $SERVICE..."
    docker compose up -d "$SERVICE"
}

start() {
    compile
    docker compose up -d --build --remove-orphans raft-log-store
}

start_release() {
    readonly RUST_BINARY_PATH="target/release/raft-log-store"
    export RUST_BINARY_PATH
    readonly RUST_LOG="INFO"
    export RUST_LOG
    compile "--release"
    docker compose up -d --build --remove-orphans raft-log-store
}

stop() {
    docker compose down
}

restart() {
    stop
    start
}

cluster_init() {
    curl -v -H "Content-Type: application/json" "http://127.0.0.1:9050/init" -d '{}' | jq
}

cluster_metrics() {
    curl -v "http://127.0.0.1:9050/metrics" | jq
}

write() {
    data="$@"
    curl -v -H "Content-Type: application/json" "http://127.0.0.1:9050/write" -d "${data}" | jq
}

read() {
    key="$@"
    curl -v -H "Content-Type: application/json" "http://127.0.0.1:9050/read" -d "\"${key}\"" | jq
}

case "$SUBCOMMAND" in
    start)
        start
        ;;
    start_release)
        start_release
        ;;
    stop)
        stop
        ;;
    recompile)
        recompile
        ;;
    restart)
        restart
        ;;
    cluster_init)
        cluster_init
        ;;
    cluster_metrics)
        cluster_metrics
        ;;
    write)
        "$SUBCOMMAND" "$@"
        ;;
    read)
        "$SUBCOMMAND" "$@"
        ;;
    *)
        # spaces for padding
        readonly s="${0//?/ }"
        cat <<EOF
Unknown option '$0 $SUBCOMMAND'

Usage:
  $0 start                        : Start all containers (dev mode)
  $0 start_release                : Start all containers (release mode)
  $0 stop                         : Stop containers
  $0 recompile                    : Recompile the service
  $0 restart                      : Restart all containers
  $0 cluster_init                 : Initialize the cluster
  $0 cluster_metrics              : Get cluster metrics
  $0 write                        : Write data to the cluster
  $0 read                         : Read data from the cluster
EOF
        exit 1
        ;;
esac
