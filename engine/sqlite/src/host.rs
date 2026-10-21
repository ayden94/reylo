//! The engine's view of a SQLite database ([`reylo_engine_core::host`]), which is never asked to
//! describe: SQLite prepares inside the engine.

use reylo_engine_core::api::NoDescribe;
pub use reylo_engine_core::host::{as_str, Lazy};

pub type Host<'a> = reylo_engine_core::host::Host<'a, NoDescribe>;
