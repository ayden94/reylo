//! The engine's view of a MySQL database ([`reylo_engine_core::host`]), whose describe is the
//! server's prepare metadata ([`MysqlDescribe`]).

use crate::api::MysqlDescribe;
pub use reylo_engine_core::host::{as_str, Lazy};

pub type Host<'a> = reylo_engine_core::host::Host<'a, MysqlDescribe>;
