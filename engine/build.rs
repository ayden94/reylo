// Compiles SQLite's amalgamation, patched with the engine's resolution hooks, together with
// the tree serializer in `csrc/reylo_sqlite.c`, for the native and the wasm32-wasip1 target alike.
// `build.sh` fetches the amalgamation and applies `csrc/patch-sqlite.py`; this script only compiles.
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// FNV-1a over the engine's sources, file by file in name order: the same on every platform
/// and toolchain, which a std hasher does not promise.
fn fingerprint(root: &Path) -> String {
    let mut files: Vec<PathBuf> = vec![root.join("Cargo.toml")];
    for dir in ["src", "csrc"] {
        // A dotfile is never a source: macOS tar adds an AppleDouble `._name` beside a file with
        // extended attributes, and a checkout on another platform would hash it.
        let mut entries: Vec<PathBuf> = fs::read_dir(root.join(dir))
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_file() && !p.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.')))
            .collect();
        entries.sort();
        files.extend(entries);
    }
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut eat = |bytes: &[u8]| {
        for b in bytes {
            if *b == b'\r' {
                continue;
            }
            hash ^= u64::from(*b);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    };
    for file in &files {
        eat(file.strip_prefix(root).unwrap_or(file).to_string_lossy().replace('\\', "/").as_bytes());
        eat(&[0]);
        eat(&fs::read(file).unwrap_or_default());
        eat(&[0]);
    }
    format!("{hash:016x}")
}

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-env=REYLO_ENGINE_FINGERPRINT={}", fingerprint(&root));
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=csrc");
    println!("cargo:rerun-if-changed=Cargo.toml");
    let patched = root.join(".sqlite/sqlite3-patched.c");
    if !patched.exists() {
        panic!("{} is missing: run `engine/build.sh --prepare` (fetches the SQLite amalgamation and applies csrc/patch-sqlite.py)", patched.display());
    }
    let target = env::var("TARGET").unwrap();
    let mut build = cc::Build::new();
    build
        .file(root.join("csrc/reylo_sqlite.c"))
        .include(root.join(".sqlite"))
        .include(root.join(".sqlite/sqlite-amalgamation-3530400"))
        .flag("-w")
        .opt_level(2)
        .define("SQLITE_THREADSAFE", "0")
        .define("SQLITE_OMIT_LOAD_EXTENSION", None)
        .define("SQLITE_ENABLE_COLUMN_METADATA", None)
        // Every optional family of SQL functions and virtual tables: the host's SQLite decides
        // which functions a statement may call (`sqlite::FUNCTIONS_QUERY`), and this build must
        // have each one some host has, to type a call to it and to replay a schema that uses it.
        .define("SQLITE_ENABLE_FTS3", None)
        .define("SQLITE_ENABLE_FTS3_PARENTHESIS", None)
        .define("SQLITE_ENABLE_FTS4", None)
        .define("SQLITE_ENABLE_FTS5", None)
        .define("SQLITE_ENABLE_RTREE", None)
        .define("SQLITE_ENABLE_GEOPOLY", None)
        .define("SQLITE_ENABLE_MATH_FUNCTIONS", None)
        .define("SQLITE_ENABLE_PERCENTILE", None)
        .define("SQLITE_ENABLE_OFFSET_SQL_FUNC", None)
        .define("SQLITE_SOUNDEX", None)
        .define("SQLITE_ENABLE_DBSTAT_VTAB", None)
        .define("SQLITE_ENABLE_DBPAGE_VTAB", None)
        .define("SQLITE_ENABLE_STMTVTAB", None)
        .define("SQLITE_ENABLE_BYTECODE_VTAB", None)
        .define("SQLITE_ENABLE_CARRAY", None)
        .define("SQLITE_DEFAULT_MEMSTATUS", "0")
        .define("SQLITE_OMIT_DEPRECATED", None)
        .define("SQLITE_OMIT_SHARED_CACHE", None)
        .define("SQLITE_TEMP_STORE", "3");
    if !target.starts_with("wasm32") {
        // A native consumer of this rlib may link its own SQLite (Reylo Framework links rusqlite's bundled
        // one), so the amalgamation's public API stays private to this translation unit: only
        // `reylo_sqlite_analyze` and `reylo_sqlite_free` are exported, and nothing here is
        // reachable from, or shadows, the other copy.
        build.define("SQLITE_API", "static").define("SQLITE_EXTERN", "");
    }
    if target.contains("linux") {
        // The amalgamation defines `_GNU_SOURCE` for itself, but `csrc/reylo_sqlite.c` has
        // included system headers before it gets there, and glibc then hides `mremap`.
        build.define("_GNU_SOURCE", None);
    }
    if target.starts_with("wasm32") {
        build
            .define("SQLITE_OMIT_WAL", None)
            .define("HAVE_FCHOWN", "0")
            .define("HAVE_READLINK", "0")
            .define("HAVE_LSTAT", "0")
            .define("SQLITE_MAX_MMAP_SIZE", "0")
            .define("SQLITE_ENABLE_LOCKING_STYLE", "0");
    }
    build.compile("reylo_sqlite");
    println!("cargo:rerun-if-changed=csrc/reylo_sqlite.c");
    println!("cargo:rerun-if-changed=csrc/patch-sqlite.py");
    println!("cargo:rerun-if-changed=.sqlite/sqlite3-patched.c");
}
