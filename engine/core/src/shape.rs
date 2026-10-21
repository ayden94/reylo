//! The vocabulary of nested JSON shapes, which every database's planner speaks: what a JSON
//! constructor builds, and the statements to describe to type its leaves. Each database plans and
//! resolves them over its own tree (`reylo-engine-postgres`'s `jsonshape`, `reylo-engine-mysql`'s
//! `mysql_shape`).

/// A value the build recovered from a JSON constructor.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// `json[b]_build_object(k, v, …)`, with the keys in the order the SQL writes them. The
    /// constructor never returns SQL NULL, so `non_null` is true unless something above it —
    /// a scalar subquery that may match no row — can null the whole value.
    Object { fields: Vec<(String, Shape)>, non_null: bool },
    /// `json[b]_agg(x)`. An aggregate over no rows is NULL, so `non_null` is true only under a
    /// `coalesce(…, '[]'::jsonb)`.
    Array { items: Box<Shape>, non_null: bool },
    /// One value expression: column `index` of probe `probe`.
    Column { probe: usize, index: usize },
    /// A whole row: every column of probe `probe`, which selects `alias.*`.
    Row { probe: usize },
    /// A bound parameter, `$n`; its type is the one the statement's own describe reported.
    Param(i32),
    /// The value of a scalar subquery that may find no row: the shape inside, or NULL.
    Nullable(Box<Shape>),
    /// Nothing the build derived.
    Unknown,
}

/// A synthesized statement whose result columns are one constructor's value expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct Probe {
    /// Over the constructor's own `FROM` clause.
    pub sql: String,
    /// The same target list over that `FROM` and every enclosing one, for a constructor that
    /// reads a correlated column the inner `FROM` alone cannot resolve. It is tried only when
    /// the narrow form fails, so a plain nested fetch costs exactly one extra describe.
    pub wider: Option<String>,
}
