//! The WebAssembly ABI: the host allocates a buffer with `reylo_alloc`, writes the request into
//! it, calls `reylo_call`, and reads a length-prefixed response (4 bytes little-endian length,
//! then the bytes) from the returned pointer, which it releases with `reylo_free`. The exports
//! belong to an artifact, not to a library: each wasm entry crate expands [`export_wasm_abi!`]
//! once, so a native host that links several databases has none of them.

pub fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len.max(1));
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// # Safety
/// `ptr` must come from [`alloc`]`(len)` or be a response pointer from [`call`], whose length is
/// the prefix plus four.
pub unsafe fn free(ptr: *mut u8, len: usize) {
    drop(Vec::from_raw_parts(ptr, 0, len.max(1)));
}

/// # Safety
/// `ptr`/`len` must describe a readable byte range holding UTF-8 JSON.
pub unsafe fn call(ptr: *const u8, len: usize, handle: fn(&str) -> String) -> *mut u8 {
    let request = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
    let response = handle(request);
    let mut out = Vec::with_capacity(4 + response.len());
    out.extend_from_slice(&(response.len() as u32).to_le_bytes());
    out.extend_from_slice(response.as_bytes());
    out.shrink_to_fit();
    let p = out.as_mut_ptr();
    std::mem::forget(out);
    p
}

/// The three exports the loader reads (`src/engine.ts`), over `handle`, one request in and one
/// response out: `export_wasm_abi!(reylo_engine_core::api::call::<reylo_engine_postgres::Postgres>)`.
#[macro_export]
macro_rules! export_wasm_abi {
    ($handle:expr) => {
        #[no_mangle]
        pub extern "C" fn reylo_alloc(len: usize) -> *mut u8 {
            $crate::abi::alloc(len)
        }

        /// # Safety
        /// As [`reylo_engine_core::abi::free`].
        #[no_mangle]
        pub unsafe extern "C" fn reylo_free(ptr: *mut u8, len: usize) {
            $crate::abi::free(ptr, len)
        }

        /// # Safety
        /// As [`reylo_engine_core::abi::call`].
        #[no_mangle]
        pub unsafe extern "C" fn reylo_call(ptr: *const u8, len: usize) -> *mut u8 {
            $crate::abi::call(ptr, len, $handle)
        }
    };
}
