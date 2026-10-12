//! Reylo's analysis engine. The JavaScript mapper hands in one JSON request and gets one JSON
//! response back; everything that touches a database happens on the JavaScript side, so the
//! engine is pure and the same artifact serves every runtime that can instantiate WebAssembly.

pub mod analyze;
pub mod api;
pub mod cardinality;
pub mod config;
pub mod dialect;
pub mod extension;
pub mod host;
pub mod infer;
pub mod jsonshape;
pub mod levels;
pub mod lex;
pub mod mysql;
pub mod mysql_params;
pub mod mysql_shape;
pub mod pgtypes;
pub mod postgres;
pub mod returning;
pub mod schema;
pub mod sqlite;
pub mod sqlite_tree;
pub mod stack;
pub mod types;

/// A fingerprint of the sources this engine was built from (`build.rs`). Two engines with the
/// same fingerprint give the same answer, which is how a client decides whether the engine
/// inside a server's `pgreylo` extension may answer for it ([`extension`]).
pub const FINGERPRINT: &str = env!("REYLO_ENGINE_FINGERPRINT");

/// One request in, one response out, both JSON text. The only entry point.
pub fn call(request: &str) -> String {
    api::call(request)
}

// The WebAssembly ABI: the host allocates a buffer with `reylo_alloc`, writes the request into
// it, calls `reylo_call`, and reads a length-prefixed response (4 bytes little-endian length,
// then the bytes) from the returned pointer, which it releases with `reylo_free`.

#[no_mangle]
pub extern "C" fn reylo_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len.max(1));
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// # Safety
/// `ptr` must come from `reylo_alloc(len)` or be a response pointer from `reylo_call`, whose
/// length is the prefix plus four.
#[no_mangle]
pub unsafe extern "C" fn reylo_free(ptr: *mut u8, len: usize) {
    drop(Vec::from_raw_parts(ptr, 0, len.max(1)));
}

/// # Safety
/// `ptr`/`len` must describe a readable byte range holding UTF-8 JSON.
#[no_mangle]
pub unsafe extern "C" fn reylo_call(ptr: *const u8, len: usize) -> *mut u8 {
    let request = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
    let response = call(request);
    let mut out = Vec::with_capacity(4 + response.len());
    out.extend_from_slice(&(response.len() as u32).to_le_bytes());
    out.extend_from_slice(response.as_bytes());
    out.shrink_to_fit();
    let p = out.as_mut_ptr();
    std::mem::forget(out);
    p
}
