// The hash of this crate's sources, its copy of `sqlparser` among them, which its fingerprint
// combines with core's (`src/lib.rs`).
fn main() {
    reylo_engine_build::emit("REYLO_ENGINE_MYSQL_HASH", &["build.rs", "src", "vendor"]);
}
