#!/usr/bin/env bash
# Builds the engine to one wasm32-wasip1 artifact, `reylo_engine.wasm`, next to this script.
#
# Inputs: a Rust toolchain with the `wasm32-wasip1` target, and wasi-sdk 34 (set WASI_SDK_PATH,
# or unpack it under `.wasi-sdk/`). The `pg_query` crate is the Postgres 18 parser: upstream's
# `pg-18` branch (pganalyze/pg_query.rs, pull request 79, "Release 18.0.0", not on crates.io yet)
# at the commit pinned below, with the libpg_query commit its submodule names (tag 18.1.0, the
# grammar of Postgres 18.6), each fetched from GitHub by commit and checked against a SHA-256, and
# the patch from `vendor/` applied into `.patched/pg_query`. SQLite is the 3.53.4 amalgamation
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
PG_QUERY_COMMIT="ad0c87ed59c84ec6b42c970bb42d7985b7b5878b"
PG_QUERY_SHA256="de2307fc9beb0c37e8da569b48d69778d5c847e8b80be2620655c032eb800043"
LIBPG_QUERY_COMMIT="c1546e7e97edc93fe8474d7a59881e91fbf48cfc"
LIBPG_QUERY_SHA256="7df58cb1d3a65fdb9fe87f64961c3b3664008fb3661d311827da47cb1d313a3e"
PG_QUERY_PATCH="vendor/pg_query-18.0.0.patch"
sha256() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi
}
# What the copy was made from: a change of either commit or of the patch makes it again.
PG_QUERY_SOURCE="$PG_QUERY_COMMIT $LIBPG_QUERY_COMMIT $(sha256 "$PG_QUERY_PATCH")"
if [ "$(cat .patched/pg_query/.source 2>/dev/null || true)" != "$PG_QUERY_SOURCE" ]; then
  # Straight from GitHub, not through `cargo fetch`: the workspace's `[patch]` points at the
  # copy this makes, so cargo resolves nothing until it exists. A commit's archive holds no
  # submodule, so libpg_query is fetched beside it, at the commit the crate's tree names.
  rm -rf .patched && mkdir -p .patched/fetch
  fetch() {
    echo "fetching $1 at $2"
    curl -fsSL -o ".patched/fetch/$1.tar.gz" "https://codeload.github.com/pganalyze/$1/tar.gz/$2"
    if [ "$(sha256 ".patched/fetch/$1.tar.gz")" != "$3" ]; then
      echo "$1 at $2: the archive's SHA-256 is $(sha256 ".patched/fetch/$1.tar.gz"), expected $3" >&2
      exit 1
    fi
    tar -xzf ".patched/fetch/$1.tar.gz" -C .patched/fetch
  }
  fetch pg_query.rs "$PG_QUERY_COMMIT" "$PG_QUERY_SHA256"
  fetch libpg_query "$LIBPG_QUERY_COMMIT" "$LIBPG_QUERY_SHA256"
  mv ".patched/fetch/pg_query.rs-$PG_QUERY_COMMIT" .patched/pg_query
  rmdir .patched/pg_query/libpg_query
  mv ".patched/fetch/libpg_query-$LIBPG_QUERY_COMMIT" .patched/pg_query/libpg_query
  # What the crate's `build.rs` never reads: libpg_query's tests and tools, and the C++ protobuf
  # bindings, 30 MB of the 36.
  (cd .patched/pg_query/libpg_query && rm -rf test tmp benchmark examples srcdata scripts patches protobuf/pg_query.pb.cc protobuf/pg_query.pb.h)
  rm -rf .patched/fetch
  (cd .patched/pg_query && patch -p1 -N < "../../$PG_QUERY_PATCH")
  echo "$PG_QUERY_SOURCE" > .patched/pg_query/.source
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
# The shadow stack, where C and the Rust frames that spill keep their locals, is 8 MiB rather than
# the 1 MiB rustc links by default: the analysis of a statement as deep as libpg_query hands over
# (src/stack.rs) needs about 0.5 MiB of it. libpg_query cannot find the bounds of a WebAssembly
# stack and keeps its default budget of 100 kB of it, which stops a statement nested about twice
# that deep while the JavaScript engine's own stack, which every wasm call also spends, has room.
export CARGO_TARGET_WASM32_WASIP1_RUSTFLAGS="-C link-arg=-L$SYSROOT/lib/wasm32-wasip1 -C link-arg=-lwasi-emulated-signal -C link-arg=-lwasi-emulated-process-clocks -C link-arg=-lwasi-emulated-mman -C link-arg=-lwasi-emulated-getpid -C link-arg=-lsetjmp -C link-arg=-zstack-size=8388608"

# The crate is a member of the workspace in `..`, so the artifact lands in the workspace's `target/`.
TARGET_DIR="${CARGO_TARGET_DIR:-$(cd .. && pwd)/target}"
# The workspace's release profile is `opt-level = 3` for Reylo Framework's server; the wasm the loader ships
# is built for size.
CARGO_PROFILE_RELEASE_OPT_LEVEL=s cargo build --release --target wasm32-wasip1 --lib
cp "$TARGET_DIR/wasm32-wasip1/release/reylo_engine.wasm" reylo_engine.wasm
ls -l reylo_engine.wasm
