//! The engine's view of a Postgres database ([`reylo_engine_core::host`]), whose describe is a
//! Parse/Describe ([`RawDescribe`]).

use crate::api::RawDescribe;
pub use reylo_engine_core::host::{as_bool, as_str, as_strings, as_u32, Lazy};

pub type Host<'a> = reylo_engine_core::host::Host<'a, RawDescribe>;
