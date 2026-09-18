//! Postgres type knowledge: the well-known OIDs the rules read, and the registry of the types a
//! statement mentions, read from `pg_type` through the host.

use crate::host::{as_strings, as_u32};
use serde_json::Value;
use std::collections::HashMap;

pub mod oid {
    pub const BOOL: u32 = 16;
    pub const INT8: u32 = 20;
    pub const INT2: u32 = 21;
    pub const INT4: u32 = 23;
    pub const TEXT: u32 = 25;
    pub const JSON: u32 = 114;
    pub const FLOAT4: u32 = 700;
    pub const FLOAT8: u32 = 701;
    pub const VARCHAR: u32 = 1043;
    pub const UNKNOWN: u32 = 705;
    pub const JSONB: u32 = 3802;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    Base,
    Enum,
    Domain,
    Array,
    Composite,
    Other,
}

#[derive(Debug, Clone)]
pub struct PgType {
    pub oid: u32,
    pub name: String,
    pub kind: TypeKind,
    pub elem: Option<u32>,
    pub base: Option<u32>,
    pub labels: Vec<String>,
    /// The name a cast may spell, schema-qualified when the search path would not find it.
    pub cast_name: String,
}

/// The types a statement mentions, by oid. Loaded lazily: the engine asks the host for the oids
/// it meets, then for the element and base types those name, until nothing is missing.
#[derive(Debug, Clone, Default)]
pub struct TypeRegistry {
    pub types: HashMap<u32, PgType>,
}

/// `$1` is `oid[]`.
pub const TYPE_QUERY: &str = "SELECT t.oid::int8, t.typname::text, t.typtype::text, t.typelem::int8, t.typbasetype::int8, \
    CASE WHEN t.typtype = 'e' THEN (SELECT array_agg(e.enumlabel::text ORDER BY e.enumsortorder) FROM pg_enum e WHERE e.enumtypid = t.oid) END, \
    pg_catalog.format_type(t.oid, NULL) \
    FROM pg_type t WHERE t.oid = ANY($1::oid[])";

impl TypeRegistry {
    /// Rows of [`TYPE_QUERY`] as the host hands them back; a driver may decode `int8` as a
    /// string, so every number is read leniently.
    pub fn insert_rows(&mut self, rows: &[Vec<Value>]) {
        for row in rows {
            let (Some(oid), Some(name), Some(typtype)) = (row.first().and_then(as_u32), row.get(1).and_then(Value::as_str), row.get(2).and_then(Value::as_str)) else {
                continue;
            };
            let elem = row.get(3).and_then(as_u32).unwrap_or(0);
            let base = row.get(4).and_then(as_u32).unwrap_or(0);
            let labels = row.get(5).and_then(as_strings);
            let cast_name = row.get(6).and_then(Value::as_str).unwrap_or("").to_string();
            let name = name.to_string();
            let kind = match typtype {
                "b" | "p" if elem != 0 && name.starts_with('_') => TypeKind::Array,
                "b" => TypeKind::Base,
                "e" => TypeKind::Enum,
                "d" => TypeKind::Domain,
                "c" => TypeKind::Composite,
                _ => TypeKind::Other,
            };
            self.types.insert(
                oid,
                PgType { oid, name, kind, elem: (elem != 0).then_some(elem), base: (base != 0).then_some(base), labels: labels.unwrap_or_default(), cast_name },
            );
        }
    }

    /// Mark an oid the catalog did not answer for, so it is not asked for again.
    pub fn insert_unknown(&mut self, oid: u32) {
        self.types.entry(oid).or_insert(PgType {
            oid,
            name: format!("oid:{oid}"),
            kind: TypeKind::Other,
            elem: None,
            base: None,
            labels: Vec::new(),
            cast_name: String::new(),
        });
    }

    pub fn get(&self, oid: u32) -> Option<&PgType> {
        self.types.get(&oid)
    }

    pub fn name(&self, oid: u32) -> String {
        self.get(oid).map(|t| t.name.clone()).unwrap_or_else(|| format!("oid:{oid}"))
    }

    /// Resolve a domain to its base type oid (transitively).
    pub fn resolve_domain(&self, oid: u32) -> u32 {
        let mut cur = oid;
        let mut hops = 0;
        while let Some(t) = self.get(cur) {
            match (t.kind, t.base) {
                (TypeKind::Domain, Some(b)) if hops < 8 => {
                    cur = b;
                    hops += 1;
                }
                _ => break,
            }
        }
        cur
    }

    /// The oids this one needs to be rendered, itself included, that the registry does not hold.
    pub fn missing(&self, oid: u32, out: &mut Vec<u32>) {
        let mut cur = oid;
        for _ in 0..8 {
            let Some(t) = self.get(cur) else {
                if !out.contains(&cur) {
                    out.push(cur);
                }
                return;
            };
            match (t.kind, t.elem, t.base) {
                (TypeKind::Array, Some(e), _) => cur = e,
                (TypeKind::Domain, _, Some(b)) => cur = b,
                _ => return,
            }
        }
    }

    /// The type model for one oid.
    pub fn render(&self, oid: u32) -> crate::types::Type {
        use crate::types::Type;
        let mut cur = oid;
        for _ in 0..8 {
            let Some(t) = self.get(cur) else { return Type::Unknown };
            match (t.kind, t.elem, t.base) {
                (TypeKind::Array, Some(e), _) => return Type::Array { of: Box::new(self.render(e)) },
                (TypeKind::Domain, _, Some(b)) => cur = b,
                (TypeKind::Enum, _, _) => return Type::Enum { name: t.name.clone(), labels: t.labels.clone() },
                (TypeKind::Base, _, _) if matches!(t.oid, oid::JSON | oid::JSONB) => return Type::Json,
                _ => return Type::Scalar { name: t.name.clone() },
            }
        }
        Type::Unknown
    }
}
