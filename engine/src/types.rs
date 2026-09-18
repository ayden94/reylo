//! The vocabulary the describe tier answers in, and the type model the engine answers with.

use serde::{Deserialize, Serialize};

/// One described result column: the server's type and its nullability as the engine settled it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribedColumn {
    pub name: String,
    pub type_oid: u32,
    pub type_name: String,
    pub nullable: bool,
}

/// One described statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DescribedStatement {
    /// Parameter type OIDs in placeholder order ($1 first).
    pub param_oids: Vec<u32>,
    pub columns: Vec<DescribedColumn>,
}

/// What the mapper renders as a TypeScript type and as JSON Schema. Inside an `Object` or a
/// `List` every value is JSON-encoded, so the same `Scalar` reads differently there (a
/// `timestamptz` is a `Date` on the wire and a string inside JSON); the renderer keeps track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Type {
    /// A base type (a domain resolves to its base) by its catalog name: `int4`, `text`, `uuid`.
    Scalar { name: String },
    /// An enum: a union of its labels.
    Enum { name: String, labels: Vec<String> },
    /// A Postgres array of the element type.
    Array { of: Box<Type> },
    /// `json[b]_build_object(…)`: the keys in the order the SQL writes them.
    Object { fields: Vec<Field> },
    /// `json[b]_agg(…)`: a JSON array of the item type.
    List { of: Box<Type>, nullable_items: bool },
    /// A `json`/`jsonb` value the engine could not explain.
    Json,
    /// Nothing the engine derived.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
    pub nullable: bool,
}
