//! `@reylosql/mysql`'s `engine.wasm`: the ABI of [`reylo_engine_core::abi`] over MySQL alone.
reylo_engine_core::export_wasm_abi!(reylo_engine_core::api::call::<reylo_engine_mysql::Mysql>);
