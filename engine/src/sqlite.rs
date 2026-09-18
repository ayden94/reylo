//! The SQLite analysis.
//!
//! SQLite prepares a statement and reports, per result column, the declared type of the table
//! column it comes from, and nothing for an expression or a parameter. The parse tree fills the
//! rest in: a parameter's context ([`crate::params`]), an expression's type from what it is
//! built of, nullability from the table's `NOT NULL` and the outer joins, and the row count from
//! the unique indexes. Every catalog fact comes through the host as a `pragma` query.

use crate::analyze::{self, Analysis as Parsed};
use crate::api::{Analysis, Column, Diagnostic, Param, Scope, SqliteDescribe};
use crate::cardinality::{self, Catalog};
use crate::dialect::Dialect;
use crate::host::{as_bool, as_str, as_u32, Host, Lazy};
use crate::infer::{self, Trust};
use crate::lex::{self, Lexed, Target};
use crate::params::{self, ColumnInfo, Hint, Tables};
use crate::pgtypes::oid;
use crate::types::{DescribedColumn, DescribedStatement, Type};
use serde_json::{json, Value};
use std::cell::Cell;
use std::collections::{HashMap, HashSet};

const MAX_PROBES: usize = 8;
const TABLE_INFO: &str = "select name, type, \"notnull\", pk from pragma_table_info(?) order by cid";
const INDEX_LIST: &str = "select name, \"unique\", origin, partial from pragma_index_list(?)";
const INDEX_INFO: &str = "select name from pragma_index_info(?) order by seqno";

/// SQLite's affinity of a declared type, as the name the type model carries.
pub fn affinity(declared: &str) -> &'static str {
    let d = declared.to_ascii_uppercase();
    if d.contains("INT") || d.contains("BOOL") {
        "integer"
    } else if d.contains("CHAR") || d.contains("CLOB") || d.contains("TEXT") {
        "text"
    } else if d.is_empty() || d.contains("BLOB") {
        "blob"
    } else if d.contains("REAL") || d.contains("FLOA") || d.contains("DOUB") {
        "real"
    } else {
        "numeric"
    }
}

fn scalar(name: &str) -> Type {
    Type::Scalar { name: name.to_string() }
}

/// A parameter hint as the type model.
fn hint_type(hint: &Hint) -> Type {
    match hint {
        Hint::Column(declared) => scalar(affinity(declared)),
        Hint::Cast(name) => scalar(match name.as_str() {
            "int2" | "int4" | "int8" | "bool" => "integer",
            "float4" | "float8" => "real",
            "text" | "varchar" | "bpchar" => "text",
            "bytea" => "blob",
            other => affinity(other),
        }),
        Hint::Integer => scalar("integer"),
        Hint::Real => scalar("real"),
        Hint::Text => scalar("text"),
    }
}

/// An expression rule's Postgres oid as a SQLite affinity.
fn oid_affinity(o: u32) -> Option<&'static str> {
    match o {
        oid::INT2 | oid::INT4 | oid::INT8 | oid::BOOL => Some("integer"),
        oid::FLOAT4 | oid::FLOAT8 => Some("real"),
        oid::TEXT | oid::VARCHAR => Some("text"),
        _ => None,
    }
}

pub fn analyze(sql: &str, lexed: &Lexed, host: &Host) -> Lazy<Analysis> {
    Engine { host, budget: Cell::new(MAX_PROBES) }.run(sql, lexed)
}

struct Engine<'a> {
    host: &'a Host<'a>,
    budget: Cell<usize>,
}

impl Engine<'_> {
    fn run(&self, sql: &str, lexed: &Lexed) -> Lazy<Analysis> {
        let mut diagnostics = Vec::new();
        // SQLite reads every placeholder style natively, so the statement is described as
        // written; libpg_query reads it with `$n`, and the runtime binds `?` by position.
        let dollar = lex::rewrite(sql, lexed, Target::Dollar);
        let raw = self.host.describe_sqlite(sql)?;
        let raw = match raw {
            Ok(r) => r,
            Err(e) => {
                diagnostics.push(Diagnostic { message: e.message, start: 0, length: sql.trim_end().len().max(1), code: 6 });
                return Some(crate::postgres::bare(sql, lexed, Dialect::Sqlite, diagnostics));
            }
        };
        let parsed = analyze::analyze(&dollar.sql).ok();
        let tree = pg_query::parse(&dollar.sql).ok().and_then(|p| serde_json::to_value(&p.protobuf).ok());
        let mut names: Vec<String> = tree.as_ref().map(params::tables_mentioned).unwrap_or_default();
        for c in &raw.columns {
            if let Some(t) = &c.table {
                if !names.contains(t) {
                    names.push(t.clone());
                }
            }
        }
        let (tables, catalog) = self.catalog(&names)?;
        let mut described = self.settle(&raw, parsed.as_ref(), &tables);
        let trust = Trust::default();
        self.refine(&dollar.sql, &mut described, &trust)?;

        // Expression columns: the type the SQL states, where the rules left none.
        let stated: Vec<Option<Hint>> = tree.as_ref().map(|t| params::result_types(t, &tables)).unwrap_or_default();
        let columns = described
            .columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                // Provenance first (the declared type), then what the rules typed, then what
                // the SQL states about the expression.
                let ty = match (raw.columns.get(i).and_then(|c| c.declared.as_deref()), col.type_oid, stated.get(i).and_then(Option::as_ref)) {
                    (Some(declared), _, _) => scalar(affinity(declared)),
                    (None, o, _) if o != oid::UNKNOWN && o != 0 => scalar(oid_affinity(o).unwrap_or("numeric")),
                    (None, _, Some(hint)) => hint_type(hint),
                    (None, _, None) => Type::Unknown,
                };
                Column { name: col.name.clone(), ty, nullable: col.nullable, type_name: col.type_name.clone() }
            })
            .collect();

        let hints = tree.as_ref().map(|t| params::infer(t, &tables)).unwrap_or_default();
        let params = lexed
            .params
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let hint = hints.get(&(i as i32 + 1));
                Param { name: name.clone(), ty: hint.map(hint_type).unwrap_or(Type::Unknown), type_name: hint.map(|h| format!("{h:?}")).unwrap_or_default() }
            })
            .collect();

        let (cardinality, reason) = match crate::postgres::cardinality_directive(lexed, &mut diagnostics) {
            Some(c) => c,
            None => self.cardinality(&dollar.sql, parsed.as_ref(), &described, &trust, &catalog),
        };
        let rewritten = lex::rewrite(sql, lexed, Target::Question);
        Some(Analysis {
            kind: parsed.as_ref().map(|p| p.statement.to_string()).unwrap_or_default(),
            params,
            columns,
            cardinality,
            cardinality_reason: reason,
            rewritten,
            style: lexed.style,
            directives: lexed.directives.clone(),
            diagnostics,
            unresolved: Vec::new(),
        })
    }

    fn cardinality(&self, sql: &str, parsed: Option<&Parsed>, described: &DescribedStatement, trust: &Trust, catalog: &Catalog) -> (String, Option<String>) {
        let Some(parsed) = parsed else { return ("many".into(), Some("the statement did not parse locally".into())) };
        if parsed.kind == analyze::StatementKind::Mutation {
            if !parsed.has_returning || described.columns.is_empty() {
                return ("none".into(), None);
            }
            return match cardinality::dml(sql, catalog) {
                Some(Ok(cardinality::Cardinality::One)) => ("one".into(), None),
                Some(Ok(cardinality::Cardinality::AtMostOne)) => ("maybeOne".into(), None),
                Some(Err(why)) => ("many".into(), Some(why)),
                None => ("many".into(), None),
            };
        }
        match cardinality::exactly_one(sql, Dialect::Sqlite, trust, catalog) {
            Ok(()) => ("one".into(), None),
            Err(_) => match cardinality::at_most_one(sql, Dialect::Sqlite, trust, catalog) {
                Ok(()) => ("maybeOne".into(), None),
                Err(why) => ("many".into(), Some(why)),
            },
        }
    }

    /// The columns of every table named, and the unique keys the row-count proof reads: the
    /// unique, non-partial indexes plus the primary key, which for an `INTEGER PRIMARY KEY` has
    /// no index of its own.
    fn catalog(&self, names: &[String]) -> Lazy<(Tables, Catalog)> {
        let mut tables = Tables::new();
        let mut catalog = Catalog::default();
        let mut pending = false;
        let mut index_names: Vec<(String, String)> = Vec::new();
        for name in names {
            let info = self.host.query(TABLE_INFO, vec![json!(name)], Scope::Statement);
            let indexes = self.host.query(INDEX_LIST, vec![json!(name)], Scope::Statement);
            let (Some(info), Some(indexes)) = (info, indexes) else {
                pending = true;
                continue;
            };
            let mut columns = Vec::new();
            let mut pk: Vec<(u32, String)> = Vec::new();
            for row in info.unwrap_or_default() {
                let (Some(col), Some(declared)) = (row.first().and_then(as_str), row.get(1).and_then(as_str)) else { continue };
                let not_null = row.get(2).and_then(as_bool).or_else(|| row.get(2).and_then(as_u32).map(|n| n != 0)).unwrap_or(false);
                let pk_pos = row.get(3).and_then(as_u32).unwrap_or(0);
                if pk_pos > 0 {
                    pk.push((pk_pos, col.to_string()));
                }
                columns.push(ColumnInfo { name: col.to_string(), declared: declared.to_string(), not_null, pk: pk_pos > 0 });
            }
            if !pk.is_empty() {
                pk.sort();
                catalog.unique_keys.entry(name.clone()).or_default().push(pk.into_iter().map(|(_, c)| c).collect());
            }
            tables.insert(name.clone(), columns);
            for row in indexes.unwrap_or_default() {
                let unique = row.get(1).and_then(as_bool).or_else(|| row.get(1).and_then(as_u32).map(|n| n != 0)).unwrap_or(false);
                let partial = row.get(3).and_then(as_bool).or_else(|| row.get(3).and_then(as_u32).map(|n| n != 0)).unwrap_or(false);
                if let (Some(index), true, false) = (row.first().and_then(as_str), unique, partial) {
                    index_names.push((name.clone(), index.to_string()));
                }
            }
        }
        for (table, index) in &index_names {
            let Some(cols) = self.host.query(INDEX_INFO, vec![json!(index)], Scope::Statement) else {
                pending = true;
                continue;
            };
            let key: Vec<Option<String>> = cols.unwrap_or_default().iter().map(|r| r.first().and_then(as_str).map(String::from)).collect();
            if let Some(key) = key.into_iter().collect::<Option<Vec<String>>>() {
                let keys = catalog.unique_keys.entry(table.clone()).or_default();
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }
        }
        if pending {
            return None;
        }
        Some((tables, catalog))
    }

    /// Nullability from the table: `NOT NULL`, or an `INTEGER PRIMARY KEY` (the rowid, never
    /// null; any other primary key column may hold NULL unless declared `NOT NULL`), unless the
    /// column's table sits on the nullable side of an outer join or the shape hides its origin.
    fn settle(&self, raw: &SqliteDescribe, parsed: Option<&Parsed>, tables: &Tables) -> DescribedStatement {
        let nullable_tables: &[String] = parsed.map(|p| p.nullable_tables.as_slice()).unwrap_or(&[]);
        let degraded = match parsed {
            Some(p) => (p.has_repeated_table && !p.nullable_tables.is_empty()) || p.has_set_op || p.has_grouping_sets,
            None => true,
        };
        let columns = raw
            .columns
            .iter()
            .map(|col| {
                let info = match (&col.table, &col.column) {
                    (Some(t), Some(c)) => tables.get(t).and_then(|cols| cols.iter().find(|x| x.name.eq_ignore_ascii_case(c))).map(|x| (t, x)),
                    _ => None,
                };
                let nullable = match info {
                    Some((table, x)) if !degraded => {
                        let rowid = x.pk && x.declared.eq_ignore_ascii_case("INTEGER") && tables.get(table).is_some_and(|cols| cols.iter().filter(|c| c.pk).count() == 1);
                        !(x.not_null || rowid) || nullable_tables.iter().any(|n| n == table)
                    }
                    _ => true,
                };
                let type_oid = if col.table.is_some() { 0 } else { oid::UNKNOWN };
                DescribedColumn { name: col.name.clone(), type_oid, type_name: col.declared.clone().unwrap_or_default(), nullable }
            })
            .collect();
        DescribedStatement { param_oids: Vec::new(), columns }
    }

    /// The expression rules, with a probe described through SQLite the way the statement was.
    fn refine(&self, sql: &str, described: &mut DescribedStatement, trust: &Trust) -> Lazy<()> {
        let plan = infer::plan(sql, Dialect::Sqlite, trust);
        let facts: Vec<Option<infer::Fact>> = match &plan.set_op {
            Some(set) => {
                let left = self.probe(&set.left, trust);
                let right = self.probe(&set.right, trust);
                let (Some(left), Some(right)) = (left, right) else { return None };
                let (Some(left), Some(right)) = (left, right) else { return Some(()) };
                let Some(facts) = infer::combine(set.kind, &left.columns, &right.columns) else { return Some(()) };
                facts.into_iter().map(Some).collect()
            }
            None => {
                let claims = plan.claims(described.columns.len());
                let open = claims.iter().zip(&described.columns).any(|(claim, col)| claim.is_some() && (col.nullable || col.type_oid == oid::UNKNOWN));
                if !open {
                    return Some(());
                }
                let leaves = match &plan.leaves {
                    Some(probe) if plan.reads_leaves() => match self.probe(probe, trust) {
                        Some(outcome) => outcome.map(|d| d.columns),
                        None => return None,
                    },
                    _ => None,
                };
                claims.iter().map(|claim| claim.map(|e| infer::resolve(e, leaves.as_deref()))).collect()
            }
        };
        if facts.len() != described.columns.len() {
            return Some(());
        }
        for (col, fact) in described.columns.iter_mut().zip(facts) {
            let Some(fact) = fact else { continue };
            if fact.non_null {
                col.nullable = false;
            }
            if let (oid::UNKNOWN, Some(type_oid)) = (col.type_oid, fact.type_oid) {
                col.type_oid = type_oid;
            }
        }
        Some(())
    }

    fn probe(&self, sql: &str, trust: &Trust) -> Lazy<Option<DescribedStatement>> {
        if self.budget.get() == 0 {
            return Some(None);
        }
        self.budget.set(self.budget.get() - 1);
        let raw = self.host.describe_sqlite(sql)?;
        let Ok(raw) = raw else { return Some(None) };
        let parsed = analyze::analyze(sql).ok();
        let mut names: Vec<String> = Vec::new();
        for c in &raw.columns {
            if let Some(t) = &c.table {
                if !names.contains(t) {
                    names.push(t.clone());
                }
            }
        }
        let (tables, _) = self.catalog(&names)?;
        let mut described = self.settle(&raw, parsed.as_ref(), &tables);
        // A leaf probe's columns carry their declared types; the rules read them as oids.
        for (col, raw_col) in described.columns.iter_mut().zip(&raw.columns) {
            if let Some(declared) = &raw_col.declared {
                col.type_oid = match affinity(declared) {
                    "integer" => oid::INT8,
                    "real" => oid::FLOAT8,
                    "text" => oid::TEXT,
                    _ => oid::UNKNOWN,
                };
            }
        }
        self.refine(sql, &mut described, trust)?;
        Some(Some(described))
    }
}

#[allow(dead_code)]
fn unused(_: &HashMap<(), ()>, _: &HashSet<()>, _: &Value) {}
