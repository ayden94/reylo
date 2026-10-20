// The hash of this crate's sources, which its fingerprint combines with core's (`src/lib.rs`).
// `vendor/` is the patch to libpg_query, which sets how deeply a statement may nest.
fn main() {
    reylo_engine_build::emit("REYLO_ENGINE_POSTGRES_HASH", &["build.rs", "src", "vendor"]);
}
