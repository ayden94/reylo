//! `engine/reylo_engine.wasm`, the all-in-one artifact the loader runs until it loads each
//! database's own package: every request goes to the engine of the dialect it names, and `ping`
//! answers a fingerprint of all three, so the loader's cache moves with any database's rules.

use reylo_engine_core::api::{call, dialect_of};
use reylo_engine_core::dialect::Dialect;
use reylo_engine_core::fingerprint::{as_str, combine, parse_hex};

const HEX: [u8; 16] = combine(&[parse_hex(reylo_engine_postgres::FINGERPRINT), parse_hex(reylo_engine_sqlite::FINGERPRINT), parse_hex(reylo_engine_mysql::FINGERPRINT)]);

fn handle(request: &str) -> String {
    if serde_json::from_str::<serde_json::Value>(request).is_ok_and(|r| r["op"] == "ping") {
        return serde_json::json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION"), "fingerprint": as_str(&HEX) }).to_string();
    }
    match dialect_of(request) {
        Some(Dialect::Sqlite) => call::<reylo_engine_sqlite::Sqlite>(request),
        Some(Dialect::Mysql) => call::<reylo_engine_mysql::Mysql>(request),
        _ => call::<reylo_engine_postgres::Postgres>(request),
    }
}

reylo_engine_core::export_wasm_abi!(handle);
