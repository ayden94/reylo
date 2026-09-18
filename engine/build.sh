#!/usr/bin/env bash
# Builds the engine to one wasm32-wasip1 artifact, `lando_engine.wasm`, next to this script.
#
# Inputs: a Rust toolchain with the `wasm32-wasip1` target, and wasi-sdk 34 (set WASI_SDK_PATH,
# or unpack it under `.wasi-sdk/`). The `pg_query` crate is the crates.io 6.2.0 source with the
# patch from `vendor/` applied into `.patched/pg_query`, which this script prepares.
set -euo pipefail
cd "$(dirname "$0")"

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

if [ ! -f .patched/pg_query/extern_block.rs ]; then
  cargo fetch
  REG=$(ls -d ~/.cargo/registry/src/*/pg_query-6.2.0 | head -1)
  rm -rf .patched && mkdir -p .patched && cp -R "$REG" .patched/pg_query
  (cd .patched/pg_query && patch -p2 -N < ../../vendor/pg_query-6.2.0-wasm.patch)
fi

export CC_wasm32_wasip1="$WASI_SDK_PATH/bin/clang"
export AR_wasm32_wasip1="$WASI_SDK_PATH/bin/llvm-ar"
export CFLAGS_wasm32_wasip1="--sysroot=$SYSROOT -I $STUBS -include $STUBS/wasi_extra.h -mllvm -wasm-enable-sjlj -D_WASI_EMULATED_SIGNAL -D_WASI_EMULATED_PROCESS_CLOCKS -D_WASI_EMULATED_MMAN -D_WASI_EMULATED_GETPID"
export BINDGEN_EXTRA_CLANG_ARGS_wasm32_wasip1="--sysroot=$SYSROOT --target=wasm32-wasip1 -I $STUBS"
export CARGO_TARGET_WASM32_WASIP1_RUSTFLAGS="-C link-arg=-L$SYSROOT/lib/wasm32-wasip1 -C link-arg=-lwasi-emulated-signal -C link-arg=-lwasi-emulated-process-clocks -C link-arg=-lwasi-emulated-mman -C link-arg=-lwasi-emulated-getpid -C link-arg=-lsetjmp"

cargo build --release --target wasm32-wasip1 --lib
cp target/wasm32-wasip1/release/lando_engine.wasm lando_engine.wasm
ls -l lando_engine.wasm
