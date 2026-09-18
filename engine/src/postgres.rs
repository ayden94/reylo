//! The Postgres analysis, round by round.
//!
//! The statement's placeholders are rewritten to `$n` and the host describes it. From the
//! describe come the parameter types and the result columns; from the catalog, `NOT NULL` for
//! every column with provenance; from the parse tree, the outer joins that null a table's
//! columns, the expression rules ([`crate::infer`]), the JSON constructors
//! ([`crate::jsonshape`]) and the row-count proof ([`crate::cardinality`]). Everything that needs
//! the database goes through [`Host`], which records what it cannot answer yet; `analyze` then
//! returns `None`, the host answers, and the next call replays with more.

use crate::analyze::{self, Analysis as Parsed};
use crate::api::{Analysis, Column, DbError, Diagnostic, Param, Scope};
use crate::cardinality::{self, Catalog};
use crate::dialect::Dialect;
use crate::host::{as_bool, as_str, as_strings, as_u32, Host, Lazy};
use crate::infer::{self, Trust};
use crate::jsonshape;
use crate::lex::{self, Lexed, Rewritten, Style, Target};
use crate::pgtypes::{TypeRegistry, TYPE_QUERY};
use crate::types::{DescribedColumn, DescribedStatement, Type};
use serde_json::{json, Value};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;

/// How many statements one analysis may synthesize to settle its columns: one per set-operation
/// arm, one per statement whose expressions read a column, one per JSON constructor context.
const MAX_PROBES: usize = 8;

const ATTNOTNULL_QUERY: &str = "SELECT a.attrelid::int8, a.attnum::int4 FROM pg_attribute a \
    JOIN unnest($1::oid[], $2::int2[]) AS u(rel, att) ON a.attrelid = u.rel AND a.attnum = u.att WHERE a.attnotnull";
const RELNAMES_QUERY: &str = "SELECT c.oid::int8, c.relname::text FROM pg_class c WHERE c.oid = ANY($1::oid[])";
/// Which of the names the expression rules read a user or an extension has defined, and which
/// built-in types a cast may target (see [`crate::infer::Trust`]).
const TRUST_QUERY: &str = "SELECT false, o.oprname::text FROM pg_operator o WHERE o.oid >= 16384 AND o.oprname::text = ANY($1::text[]) \
    UNION ALL SELECT false, p.proname::text FROM pg_proc p WHERE p.oid >= 16384 AND p.proname::text = ANY($2::text[]) \
    UNION ALL SELECT true, t.typname::text FROM pg_type t WHERE t.oid < 16384 AND t.typnamespace = 'pg_catalog'::regnamespace \
      AND NOT EXISTS (SELECT 1 FROM pg_type u WHERE u.oid >= 16384 AND u.typname = t.typname) \
      AND NOT EXISTS (SELECT 1 FROM pg_cast c WHERE c.oid >= 16384 AND c.casttarget = t.oid)";

pub fn analyze(sql: &str, dialect: Dialect, host: &Host) -> Lazy<Analysis> {
    let lexed = match lex::lex(sql, dialect) {
        Ok(l) => l,
        Err(e) => return Some(bare(sql, &Lexed { params: vec![], placeholders: vec![], directives: vec![], style: Style::None }, dialect, vec![diag(e.message, e.offset, 1, 1)])),
    };
    if lexed.style == Style::Mixed {
        let first = lexed.placeholders.iter().find(|p| lex::is_positional(&p.name)).expect("a mixed statement has a positional placeholder");
        let d = diag("Named and positional parameters cannot be mixed in one statement.", first.start, first.end - first.start, 2);
        return Some(bare(sql, &lexed, dialect, vec![d]));
    }
    if dialect == Dialect::Sqlite {
        return crate::sqlite::analyze(sql, &lexed, host);
    }
    if dialect != Dialect::Postgres {
        return Some(bare(sql, &lexed, dialect, Vec::new()));
    }
    let engine = Engine::new(host);
    engine.run(sql, &lexed)
}

fn diag(message: impl Into<String>, start: usize, length: usize, code: u32) -> Diagnostic {
    Diagnostic { message: message.into(), start, length: length.max(1), code }
}

/// What the engine answers when it cannot describe: every parameter unknown, no columns, the
/// cardinality the directive names or `many`.
pub(crate) fn bare(sql: &str, lexed: &Lexed, dialect: Dialect, mut diagnostics: Vec<Diagnostic>) -> Analysis {
    let target = if dialect == Dialect::Postgres { Target::Dollar } else { Target::Question };
    let rewritten = lex::rewrite(sql, lexed, target);
    let (cardinality, _) = cardinality_directive(lexed, &mut diagnostics).unwrap_or(("many".to_string(), None));
    Analysis {
        kind: String::new(),
        params: lexed.params.iter().map(|n| Param { name: n.clone(), ty: Type::Unknown, type_name: String::new() }).collect(),
        columns: Vec::new(),
        cardinality,
        cardinality_reason: None,
        rewritten,
        style: lexed.style,
        directives: lexed.directives.clone(),
        diagnostics,
        unresolved: Vec::new(),
    }
}

/// `-- @returns one|maybeOne|many|none`, when the statement carries one.
pub(crate) fn cardinality_directive(lexed: &Lexed, diagnostics: &mut Vec<Diagnostic>) -> Option<(String, Option<String>)> {
    let d = lexed.directives.iter().find(|d| d.name == "returns")?;
    if ["many", "one", "maybeOne", "none"].contains(&d.value.as_str()) {
        Some((d.value.clone(), None))
    } else {
        diagnostics.push(diag(format!("Unknown @returns value \"{}\"; expected many, one, maybeOne or none.", d.value), d.start, d.end - d.start, 3));
        None
    }
}

struct Engine<'a> {
    host: &'a Host<'a>,
    registry: RefCell<TypeRegistry>,
    /// Per placeholder of the main statement, the cast a probe pins on it.
    casts: RefCell<Vec<Option<String>>>,
    budget: Cell<usize>,
}

impl<'a> Engine<'a> {
    fn new(host: &'a Host<'a>) -> Engine<'a> {
        Engine { host, registry: RefCell::new(TypeRegistry::default()), casts: RefCell::new(Vec::new()), budget: Cell::new(MAX_PROBES) }
    }

    fn run(&self, sql: &str, lexed: &Lexed) -> Lazy<Analysis> {
        let mut diagnostics = Vec::new();
        let rewritten = lex::rewrite(sql, lexed, Target::Dollar);
        // The session facts and the describe are independent: ask for both in the first round.
        let trust = self.trust();
        let operators = self.set_returning_operators();
        let raw = self.host.describe(&rewritten.sql);
        let (Some(raw), Some(trust), Some(operators)) = (raw, trust, operators) else { return None };
        let raw = match raw {
            Ok(r) => r,
            Err(e) => {
                diagnostics.push(db_diagnostic(sql, &rewritten, &e));
                return Some(bare(sql, lexed, Dialect::Postgres, diagnostics));
            }
        };
        let parsed = analyze::analyze(&rewritten.sql).ok();
        // Types for the parameters and the columns, and the casts probes pin on the parameters.
        let mut oids: Vec<u32> = raw.params.clone();
        oids.extend(raw.columns.iter().map(|c| c.oid));
        let types = self.types(&oids);
        // The catalog the row-count proof reads.
        let catalog = self.catalog(parsed.as_ref(), &operators);
        // Nullability from provenance, then from the SQL.
        let settled = self.settle(&raw, parsed.as_ref());
        let (Some(()), Some(catalog), Some(mut described)) = (types, catalog, settled) else { return None };
        *self.casts.borrow_mut() = raw.params.iter().map(|o| self.registry.borrow().get(*o).map(|t| t.cast_name.clone()).filter(|c| !c.is_empty())).collect();
        self.refine(&rewritten.sql, &mut described, &trust)?;

        // JSON shapes.
        let plan = jsonshape::plan(&rewritten.sql);
        let mut outcomes: Vec<Option<jsonshape::Outcome>> = Vec::with_capacity(plan.probes.len());
        let mut pending = false;
        for probe in &plan.probes {
            let mut outcome = self.probe(&probe.sql, &trust);
            if let (Some(None), Some(wider)) = (&outcome, &probe.wider) {
                outcome = self.probe(wider, &trust);
            }
            match outcome {
                Some(Some(d)) => {
                    let outer_join = analyze::analyze(&probe.sql).map(|a| a.has_outer_join).unwrap_or(true);
                    outcomes.push(Some(jsonshape::Outcome { columns: d.columns, outer_join }));
                }
                Some(None) => outcomes.push(None),
                None => {
                    pending = true;
                    outcomes.push(None);
                }
            }
        }
        if pending {
            return None;
        }
        let mut shape_oids = Vec::new();
        for (_, shape) in &plan.columns {
            jsonshape::oids(shape, &outcomes, &raw.params, &mut shape_oids);
        }
        self.types(&shape_oids)?;

        let registry = self.registry.borrow();
        let mut unresolved = Vec::new();
        let mut columns = Vec::with_capacity(described.columns.len());
        let mut shapes_used: HashSet<usize> = HashSet::new();
        for col in &described.columns {
            let shape = plan.columns.iter().enumerate().find(|(i, (n, _))| n == &col.name && !shapes_used.contains(i));
            let (ty, nullable) = match shape {
                Some((i, (_, shape))) if jsonshape::is_json(&registry, col.type_oid) => {
                    shapes_used.insert(i);
                    let resolved = jsonshape::resolve(shape, &outcomes, &registry, &raw.params);
                    unresolved.extend(resolved.unresolved.iter().map(|p| format!("{}: {p}", col.name)));
                    (resolved.ty, !resolved.non_null)
                }
                _ => (registry.render(col.type_oid), col.nullable),
            };
            columns.push(Column { name: col.name.clone(), ty, nullable, type_name: col.type_name.clone() });
        }
        let params = lexed
            .params
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let o = raw.params.get(i).copied().unwrap_or(0);
                Param { name: name.clone(), ty: registry.render(o), type_name: registry.name(o) }
            })
            .collect();

        // Row count.
        let (cardinality, reason) = match cardinality_directive(lexed, &mut diagnostics) {
            Some(c) => c,
            None => self.cardinality(&rewritten.sql, parsed.as_ref(), &described, &trust, &catalog),
        };

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
            unresolved,
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
        match cardinality::exactly_one(sql, Dialect::Postgres, trust, catalog) {
            Ok(()) => ("one".into(), None),
            Err(_) => match cardinality::at_most_one(sql, Dialect::Postgres, trust, catalog) {
                Ok(()) => ("maybeOne".into(), None),
                Err(why) => ("many".into(), Some(why)),
            },
        }
    }

    // -----------------------------------------------------------------------------------------
    // Session facts
    // -----------------------------------------------------------------------------------------

    fn trust(&self) -> Lazy<Trust> {
        let operators: Vec<Value> = infer::POSTGRES_OPERATORS.iter().map(|s| json!(s)).collect();
        let functions: Vec<Value> = infer::FUNCTIONS.iter().map(|s| json!(s)).collect();
        let rows = self.host.query(TRUST_QUERY, vec![json!(operators), json!(functions)], Scope::Session)?;
        let mut trust = Trust { shadowed: HashSet::new(), cast_targets: Some(HashSet::new()) };
        if let Ok(rows) = rows {
            for row in rows {
                let (Some(cast), Some(name)) = (row.first().and_then(as_bool), row.get(1).and_then(as_str)) else { continue };
                if cast {
                    trust.cast_targets.get_or_insert_with(HashSet::new).insert(name.to_string());
                } else {
                    trust.shadowed.insert(name.to_string());
                }
            }
        }
        Some(trust)
    }

    fn set_returning_operators(&self) -> Lazy<HashSet<String>> {
        let rows = self.host.query(cardinality::SET_RETURNING_OPERATORS_QUERY, vec![], Scope::Session)?;
        Some(rows.unwrap_or_default().iter().filter_map(|r| r.first().and_then(as_str).map(String::from)).collect())
    }

    // -----------------------------------------------------------------------------------------
    // Catalog
    // -----------------------------------------------------------------------------------------

    /// Load the types `oids` need, element and base types included, asking the host for the
    /// ones the registry lacks. One round per level of nesting.
    fn types(&self, oids: &[u32]) -> Lazy<()> {
        loop {
            let mut missing = Vec::new();
            {
                let registry = self.registry.borrow();
                for o in oids {
                    if *o != 0 {
                        registry.missing(*o, &mut missing);
                    }
                }
            }
            if missing.is_empty() {
                return Some(());
            }
            missing.sort_unstable();
            let rows = self.host.query(TYPE_QUERY, vec![json!(missing)], Scope::Statement)?;
            let mut registry = self.registry.borrow_mut();
            registry.insert_rows(&rows.unwrap_or_default());
            for o in &missing {
                registry.insert_unknown(*o);
            }
        }
    }

    fn catalog(&self, parsed: Option<&Parsed>, operators: &HashSet<String>) -> Lazy<Catalog> {
        let mut catalog = Catalog { set_returning_operators: operators.clone(), ..Catalog::default() };
        let Some(parsed) = parsed else { return Some(catalog) };
        let mut relations: Vec<String> = parsed.read_set.clone();
        for w in &parsed.write_set {
            if !relations.contains(w) {
                relations.push(w.clone());
            }
        }
        let keys = if relations.is_empty() { Some(Ok(Vec::new())) } else { self.host.query(cardinality::UNIQUE_KEYS_QUERY, vec![json!(relations)], Scope::Statement) };
        let functions = if parsed.functions.is_empty() {
            Some(Ok(Vec::new()))
        } else {
            self.host.query(cardinality::FUNCTIONS_QUERY, vec![json!(parsed.functions)], Scope::Statement)
        };
        let (Some(keys), Some(functions)) = (keys, functions) else { return None };
        for row in keys.unwrap_or_default() {
            if let (Some(rel), Some(cols)) = (row.first().and_then(as_str), row.get(1).and_then(as_strings)) {
                catalog.unique_keys.entry(rel.to_string()).or_default().push(cols);
            }
        }
        for row in functions.unwrap_or_default() {
            if let (Some(name), Some(retset)) = (row.first().and_then(as_str), row.get(1).and_then(as_bool)) {
                if retset {
                    catalog.set_returning.insert(name.to_string());
                }
                catalog.functions.insert(name.to_string());
            }
        }
        Some(catalog)
    }

    // -----------------------------------------------------------------------------------------
    // Describe, settle, refine
    // -----------------------------------------------------------------------------------------

    /// A described statement's columns with their nullability from provenance: the catalog's
    /// `NOT NULL`, unless the column's table sits on the nullable side of an outer join, or the
    /// statement's shape hides which relation produced a column.
    fn settle(&self, raw: &crate::api::RawDescribe, parsed: Option<&Parsed>) -> Lazy<DescribedStatement> {
        let pairs: Vec<(u32, i32)> = raw.columns.iter().filter(|c| c.table != 0 && c.attnum > 0).map(|c| (c.table, c.attnum)).collect();
        let rels: Vec<u32> = {
            let mut r: Vec<u32> = pairs.iter().map(|(r, _)| *r).collect();
            r.sort_unstable();
            r.dedup();
            r
        };
        let not_null = if pairs.is_empty() {
            Some(Ok(Vec::new()))
        } else {
            let rs: Vec<u32> = pairs.iter().map(|(r, _)| *r).collect();
            let atts: Vec<i32> = pairs.iter().map(|(_, a)| *a).collect();
            self.host.query(ATTNOTNULL_QUERY, vec![json!(rs), json!(atts)], Scope::Statement)
        };
        let nullable_tables: &[String] = parsed.map(|p| p.nullable_tables.as_slice()).unwrap_or(&[]);
        let relnames = if nullable_tables.is_empty() || rels.is_empty() {
            Some(Ok(Vec::new()))
        } else {
            self.host.query(RELNAMES_QUERY, vec![json!(rels)], Scope::Statement)
        };
        let (Some(not_null), Some(relnames)) = (not_null, relnames) else { return None };
        let not_null: HashSet<(u32, i32)> = not_null
            .unwrap_or_default()
            .iter()
            .filter_map(|r| Some((as_u32(r.first()?)?, as_u32(r.get(1)?)? as i32)))
            .collect();
        let nulled_rels: HashSet<u32> = relnames
            .unwrap_or_default()
            .iter()
            .filter_map(|r| {
                let (o, name) = (as_u32(r.first()?)?, as_str(r.get(1)?)?);
                nullable_tables.iter().any(|t| t == name).then_some(o)
            })
            .collect();
        // Without a parse tree nothing degrades and nothing is nulled by a join, which is
        // wrong in the unsafe direction, so a statement libpg_query cannot read keeps every
        // column nullable.
        let degraded = match parsed {
            Some(p) => (p.has_repeated_table && !p.nullable_tables.is_empty()) || p.has_set_op || p.has_grouping_sets,
            None => true,
        };
        let registry = self.registry.borrow();
        let columns = raw
            .columns
            .iter()
            .map(|col| {
                let nullable = if col.table != 0 && col.attnum > 0 && !degraded {
                    !not_null.contains(&(col.table, col.attnum)) || nulled_rels.contains(&col.table)
                } else {
                    true
                };
                DescribedColumn { name: col.name.clone(), type_oid: col.oid, type_name: registry.name(col.oid), nullable }
            })
            .collect();
        Some(DescribedStatement { param_oids: raw.params.clone(), columns })
    }

    /// Settle what the describe left open about expression columns from the SQL itself. A column
    /// only ever becomes non-null here, and a probe that cannot be described proves nothing.
    fn refine(&self, sql: &str, described: &mut DescribedStatement, trust: &Trust) -> Lazy<()> {
        let plan = infer::plan(sql, Dialect::Postgres, trust);
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
                let open = claims.iter().zip(&described.columns).any(|(claim, col)| claim.is_some() && col.nullable);
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
            if let Some(fact) = fact {
                if fact.non_null {
                    col.nullable = false;
                }
            }
        }
        Some(())
    }

    /// Describe a synthesized statement the way the main one is described, refinement included.
    /// `Some(None)` is a probe the database refused, which proves nothing.
    fn probe(&self, sql: &str, trust: &Trust) -> Lazy<Option<DescribedStatement>> {
        if self.budget.get() == 0 {
            return Some(None);
        }
        self.budget.set(self.budget.get() - 1);
        let pinned = self.pin(sql);
        let raw = self.host.describe(&pinned)?;
        let Ok(raw) = raw else { return Some(None) };
        let parsed = analyze::analyze(sql).ok();
        let mut oids: Vec<u32> = raw.columns.iter().map(|c| c.oid).collect();
        oids.extend(raw.params.iter().copied());
        let types = self.types(&oids);
        let settled = self.settle(&raw, parsed.as_ref());
        let (Some(()), Some(mut described)) = (types, settled) else { return None };
        self.refine(sql, &mut described, trust)?;
        Some(Some(described))
    }

    /// A probe reads the main statement's placeholders, and the server cannot type `$2` in a
    /// statement that never mentions `$1`: pin the type the main describe settled on each.
    fn pin(&self, sql: &str) -> String {
        let Ok(lexed) = lex::lex(sql, Dialect::Postgres) else { return sql.to_string() };
        if lexed.placeholders.is_empty() {
            return sql.to_string();
        }
        let casts = self.casts.borrow();
        lex::rewrite_with(sql, &lexed, Target::Dollar, &|name, rendered| {
            let cast = name.strip_prefix('$').and_then(|n| n.parse::<usize>().ok()).and_then(|n| casts.get(n.wrapping_sub(1)).cloned().flatten());
            match cast {
                Some(c) => format!("({rendered}::{c})"),
                None => rendered.to_string(),
            }
        })
        .sql
    }
}

/// A failed describe as a diagnostic on the author's text: the server's message, at the
/// characters its position points at.
fn db_diagnostic(sql: &str, rewritten: &Rewritten, e: &DbError) -> Diagnostic {
    let start = e
        .position
        .filter(|p| *p > 0)
        .and_then(|p| rewritten.sql.char_indices().nth(p - 1).map(|(b, _)| b))
        .map(|b| rewritten.to_original(b))
        .unwrap_or(0)
        .min(sql.len());
    let length = sql[start..].bytes().take_while(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.' | b'"' | b'$' | b':' | b'@')).count();
    let message = match &e.code {
        Some(code) => format!("{} ({code})", e.message),
        None => e.message.clone(),
    };
    diag(message, start, length, 6)
}
