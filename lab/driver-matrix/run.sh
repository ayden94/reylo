#!/usr/bin/env bash
# Runs the whole driver matrix: starts and seeds the servers (setup.sh), type-checks every client
# against ClientFor<D> (typecheck/check.ts), then runs every driver job in its own process
# (runner.ts) and writes results.json and RESULTS.md. Extra arguments go to runner.ts
# (--only <substring>, --jobs <n>).
set -uo pipefail
cd "$(dirname "$0")"
NODE26="${DM_NODE26:-$HOME/.nvm/versions/node/v26.7.0/bin/node}"
mkdir -p .out

if [ ! -d node_modules ]; then
  nub install > .out/install.log 2>&1
  status=$?
  if [ $status -ne 0 ]; then cat .out/install.log; echo "run.sh: nub install failed (exit $status)"; exit $status; fi
fi

./setup.sh > .out/setup.log 2>&1
status=$?
cat .out/setup.log
if [ $status -ne 0 ]; then echo "run.sh: setup.sh failed (exit $status)"; exit $status; fi

"$NODE26" typecheck/check.ts > .out/typecheck.log 2>&1
status=$?
tail -3 .out/typecheck.log
if [ $status -ne 0 ]; then echo "run.sh: the type check harness failed (exit $status); see .out/typecheck.log"; fi

"$NODE26" runner.ts "$@"
