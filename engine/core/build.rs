// The hash of core's sources, which every database's fingerprint combines with its own
// (`src/fingerprint.rs`).
fn main() {
    reylo_engine_build::emit("REYLO_ENGINE_CORE_HASH", &["build.rs", "src"]);
}
