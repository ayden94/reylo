//! The vocabulary the describe tier answers in, and the type model the engine answers with.

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
    /// A Postgres array of the element type. Any array may hold NULL elements, whatever a
    /// column's `NOT NULL`, yet an array is typed `T[]` all the same, as the maintainer decided on
    /// 2026-10-03 (`docs/driver-decoding.md`). Only an array the statement builds says more
    /// ([`DescribedColumn::nullable_items`]): `Some(true)` when an element may be NULL, as
    /// `array_agg(u.email)` over a LEFT JOIN gives `{NULL}` for a team with no users, which
    /// renders `(T | null)[]`; `Some(false)` when every element is proven non-null.
    Array {
        of: Box<Type>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        nullable_items: Option<bool>,
    },
    /// `json[b]_build_object(…)`: the keys in the order the SQL writes them.
    Object { fields: Vec<Field> },
    /// A named composite (a table's row type, a `CREATE TYPE … AS (…)`) whose columns the
    /// catalog gave: each field nullable unless the column is `NOT NULL`. Unlike an `Object`, its
    /// fields are read in the position the value itself stands in: a driver hands the whole value
    /// over as the text Postgres prints, and inside a JSON document the database writes it as an
    /// object of its fields. A composite whose columns were not read stays `Scalar { "record" }`.
    Composite { name: String, fields: Vec<Field> },
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
