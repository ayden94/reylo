#!/usr/bin/env bash
# Builds the engine to one wasm32-wasip1 artifact, `reylo_engine.wasm`, next to this script.
#
# Inputs: a Rust toolchain with the `wasm32-wasip1` target, and wasi-sdk 34 (set WASI_SDK_PATH,
# or unpack it under `.wasi-sdk/`). The `pg_query` crate is the crates.io 6.2.0 source with the
# patch from `vendor/` applied into `.patched/pg_query`, and SQLite is the 3.53.4 amalgamation
# fetched into `.sqlite/` with the hooks of `csrc/patch-sqlite.py` applied; this script
# prepares both. `./build.sh --prepare` stops after that, which is what a native `cargo test` needs,
# and what any cargo command in the workspace needs first (the workspace `[patch]` points at the copy).
set -euo pipefail
cd "$(dirname "$0")"

SQLITE_ZIP="sqlite-amalgamation-3530400.zip"
SQLITE_URL="https://sqlite.org/2026/$SQLITE_ZIP"
if [ ! -f .sqlite/sqlite-amalgamation-3530400/sqlite3.c ]; then
  mkdir -p .sqlite
  echo "fetching $SQLITE_URL"
  curl -fsSL -o ".sqlite/$SQLITE_ZIP" "$SQLITE_URL"
  (cd .sqlite && unzip -q -o "$SQLITE_ZIP")
fi
if [ ! -f .sqlite/sqlite3-patched.c ] || [ csrc/patch-sqlite.py -nt .sqlite/sqlite3-patched.c ]; then
  python3 csrc/patch-sqlite.py .sqlite/sqlite-amalgamation-3530400/sqlite3.c .sqlite/sqlite3-patched.c
fi
if [ ! -f .patched/pg_query/extern_block.rs ]; then
  # Straight from crates.io, not through `cargo fetch`: the workspace's `[patch]` points at the
  # copy this makes, so cargo resolves nothing until it exists.
  echo "fetching pg_query 6.2.0"
  rm -rf .patched && mkdir -p .patched
  curl -fsSL https://static.crates.io/crates/pg_query/pg_query-6.2.0.crate | tar -xz -C .patched
  mv .patched/pg_query-6.2.0 .patched/pg_query
  (cd .patched/pg_query && patch -p2 -N < ../../vendor/pg_query-6.2.0-wasm.patch)
fi
if [ "${1:-}" = "--prepare" ]; then
  exit 0
fi

if [ -z "${WASI_SDK_PATH:-}" ]; then
  WASI_SDK_PATH=$(ls -d .wasi-sdk/wasi-sdk-* 2>/dev/null | head -1 || true)
fi
if [ -z "$WASI_SDK_PATH" ] || [ ! -x "$WASI_SDK_PATH/bin/clang" ]; then
  echo "wasi-sdk not found: set WASI_SDK_PATH or unpack wasi-sdk 34 under engine/.wasi-sdk/" >&2
  exit 1
fi
WASI_SDK_PATH=$(cd "$WASI_SDK_PATH" && pwd)
SYSROOT="$WASI_SDK_PATH/share/wasi-sysroot"
STUBS="$(pwd)/vendor/stubs"


export CC_wasm32_wasip1="$WASI_SDK_PATH/bin/clang"
export AR_wasm32_wasip1="$WASI_SDK_PATH/bin/llvm-ar"
export CFLAGS_wasm32_wasip1="--sysroot=$SYSROOT -I $STUBS -include $STUBS/wasi_extra.h -mllvm -wasm-enable-sjlj -D_WASI_EMULATED_SIGNAL -D_WASI_EMULATED_PROCESS_CLOCKS -D_WASI_EMULATED_MMAN -D_WASI_EMULATED_GETPID"
export BINDGEN_EXTRA_CLANG_ARGS_wasm32_wasip1="--sysroot=$SYSROOT --target=wasm32-wasip1 -I $STUBS"
export CARGO_TARGET_WASM32_WASIP1_RUSTFLAGS="-C link-arg=-L$SYSROOT/lib/wasm32-wasip1 -C link-arg=-lwasi-emulated-signal -C link-arg=-lwasi-emulated-process-clocks -C link-arg=-lwasi-emulated-mman -C link-arg=-lwasi-emulated-getpid -C link-arg=-lsetjmp"

# The crate is a member of the workspace in `..`, so the artifact lands in the workspace's `target/`.
TARGET_DIR="${CARGO_TARGET_DIR:-$(cd .. && pwd)/target}"
# The workspace's release profile is `opt-level = 3` for Reylo Framework's server; the wasm the loader ships
# is built for size.
CARGO_PROFILE_RELEASE_OPT_LEVEL=s cargo build --release --target wasm32-wasip1 --lib
cp "$TARGET_DIR/wasm32-wasip1/release/reylo_engine.wasm" reylo_engine.wasm
ls -l reylo_engine.wasm
