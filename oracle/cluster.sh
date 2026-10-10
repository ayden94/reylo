#!/usr/bin/env bash
# A throwaway Postgres cluster for the oracle, under /tmp and on a port of its own, never the
# shared server:
#
#   oracle/cluster.sh start /opt/homebrew/opt/postgresql@18/bin 55418   # prints the URL
#   oracle/cluster.sh stop 55418                                        # stops it and deletes it
set -euo pipefail
case "${1:-}" in
  start)
    BIN="${2:?the bin directory of a Postgres install}"
    PORT="${3:?a free port}"
    DIR="/tmp/reylo-oracle-$PORT"
    if [ ! -d "$DIR" ]; then
      "$BIN/initdb" -D "$DIR" -U postgres --auth=trust > "$DIR.initdb.log" 2>&1
      echo "$BIN" > "$DIR/oracle-bin"
    fi
    "$BIN/pg_ctl" -D "$DIR" -o "-p $PORT -k /tmp" -l "$DIR.log" -w start > /dev/null
    echo "postgres://postgres@localhost:$PORT/postgres"
    ;;
  stop)
    PORT="${2:?the port the cluster runs on}"
    DIR="/tmp/reylo-oracle-$PORT"
    [ -d "$DIR" ] || { echo "no cluster at $DIR"; exit 0; }
    "$(cat "$DIR/oracle-bin")/pg_ctl" -D "$DIR" -m fast stop > /dev/null || true
    rm -rf "$DIR" "$DIR.log" "$DIR.initdb.log"
    ;;
  *)
    echo "usage: $0 start <postgres bin dir> <port> | stop <port>" >&2
    exit 2
    ;;
esac
