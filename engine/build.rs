// Compiles SQLite's amalgamation, patched with the engine's five resolution hooks, together with
// the tree serializer in `csrc/lando_sqlite.c`, for the native and the wasm32-wasip1 target alike.
// `build.sh` fetches the amalgamation and applies `csrc/patch-sqlite.py`; this script only compiles.
use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let patched = root.join(".sqlite/sqlite3-patched.c");
    if !patched.exists() {
        panic!("{} is missing: run `engine/build.sh --prepare` (fetches the SQLite amalgamation and applies csrc/patch-sqlite.py)", patched.display());
    }
    let target = env::var("TARGET").unwrap();
    let mut build = cc::Build::new();
    build
        .file(root.join("csrc/lando_sqlite.c"))
        .include(root.join(".sqlite"))
        .include(root.join(".sqlite/sqlite-amalgamation-3530400"))
        .flag("-w")
        .opt_level(2)
        .define("SQLITE_THREADSAFE", "0")
        .define("SQLITE_OMIT_LOAD_EXTENSION", None)
        .define("SQLITE_ENABLE_COLUMN_METADATA", None)
        .define("SQLITE_ENABLE_FTS5", None)
        .define("SQLITE_ENABLE_RTREE", None)
        .define("SQLITE_ENABLE_MATH_FUNCTIONS", None)
        .define("SQLITE_DEFAULT_MEMSTATUS", "0")
        .define("SQLITE_OMIT_DEPRECATED", None)
        .define("SQLITE_OMIT_SHARED_CACHE", None)
        .define("SQLITE_TEMP_STORE", "3");
    if target.starts_with("wasm32") {
        build
            .define("SQLITE_OMIT_WAL", None)
            .define("HAVE_FCHOWN", "0")
            .define("HAVE_READLINK", "0")
            .define("HAVE_LSTAT", "0")
            .define("SQLITE_MAX_MMAP_SIZE", "0")
            .define("SQLITE_ENABLE_LOCKING_STYLE", "0");
    }
    build.compile("lando_sqlite");
    println!("cargo:rerun-if-changed=csrc/lando_sqlite.c");
    println!("cargo:rerun-if-changed=csrc/patch-sqlite.py");
    println!("cargo:rerun-if-changed=.sqlite/sqlite3-patched.c");
}
