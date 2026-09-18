# Engine build inputs (WebAssembly)

Files preserved from the packaging research (`docs/rust-engine.md`, section 7) so the `wasm32-wasip1` build of the `pg_query` crate (libpg_query inside) is reproducible:

- `pg_query-6.2.0-wasm.patch`: the diff against the crates.io `pg_query` 6.2.0 sources: header shims for the wasm target, a `sigsetjmp` to `setjmp` mapping, and the `build.rs` change that pastes in the `extern "C"` block bindgen drops on wasm targets (`pg_query-extern_block.rs` is that block; `pg_query-build.rs` is the full patched `build.rs`).
- `stubs/`: empty or minimal headers (`syslog.h`, `netdb.h`, `arpa/`, `netinet/`, `sys/`, `wasi_extra.h`) that libpg_query includes but WASI does not provide.
- `Cargo.toml` and `src/main.rs`: the probe crate that proved parse, deparse and the `PG_TRY`/`longjmp` error path under Node's WASI.

Toolchain: `wasi-sdk` 34 (`CC` and `AR` from its `bin/`, `--sysroot` from its `share/wasi-sysroot`, `-I stubs`), `rustup target add wasm32-wasip1`, `cargo build --release --target wasm32-wasip1`. The engine crate proper will live next to this directory.
