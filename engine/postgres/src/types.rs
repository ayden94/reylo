//! The vocabulary the Postgres describe tier answers in, beside the type model every database
//! shares ([`reylo_engine_core::types`]).

pub use reylo_engine_core::types::*;
use serde::{Deserialize, Serialize};

/// One described result column: the server's type and its nullability as the engine settled it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribedColumn {
    pub name: String,
    pub type_oid: u32,
    pub type_name: String,
    pub nullable: bool,
    /// For an array column the statement builds (`ARRAY[a, b]`, `array_agg(x)`, an array
    /// literal), whether an element may be NULL; `None` for every other column, a table's array
    /// column and a function's array result among them ([`Type::Array`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable_items: Option<bool>,
    /// For a bare read of a string column a CHECK keeps to a list (`status IN ('active',
    /// 'banned')`), those strings: its values are a union of literals, as an enum's labels are.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    /// For a bare read of a `json` or `jsonb` column a CHECK holds to a JSON Schema the database
    /// validates (`jsonb_matches_schema('…', doc)`), the canonical schema of the values it admits
    /// ([`crate::schema_type`]): its values are those, as [`Type::Json`]'s `schema` says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<serde_json::Value>,
    /// Whether the row value the column holds, or each one of an array of them, is a stored row
    /// of the table whose row type it is: a whole row the statement reads (`d` of `from docs d`,
    /// through a view, a derived table or a WITH query too), or one a scalar or `ARRAY` subquery
    /// returns (`array(select c from child c …)`). Such a row met the table's `NOT NULL` and
    /// CHECKs; a row value the statement builds (`row(…)::docs`), a function returns or another
    /// table's column holds only shares the type.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub whole_row: bool,
}

/// One described statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DescribedStatement {
    /// Parameter type OIDs in placeholder order ($1 first).
    pub param_oids: Vec<u32>,
    pub columns: Vec<DescribedColumn>,
}
