//! `@reylosql/postgres`'s `engine.wasm`: the ABI of [`reylo_engine_core::abi`] over Postgres alone.
reylo_engine_core::export_wasm_abi!(reylo_engine_core::api::call::<reylo_engine_postgres::Postgres>);
