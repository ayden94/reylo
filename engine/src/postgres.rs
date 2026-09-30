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
use crate::api::{Analysis, Column, DbError, Diagnostic, Param, RawDescribe, Scope};
use crate::cardinality::{self, Catalog};
use crate::dialect::Dialect;
use crate::host::{as_bool, as_str, as_strings, as_u32, Host, Lazy};
use crate::infer::{self, Trust};
use crate::jsonshape;
use crate::lex::{self, Lexed, Rewritten, Style, Target};
use crate::pgtypes::{TypeRegistry, RELATION_TYPES_QUERY, TYPE_QUERY};
use crate::types::{DescribedColumn, DescribedStatement, Type};
use serde_json::{json, Value};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet};

/// How many statements one analysis may synthesize to settle its columns: one per set-operation
/// arm, one per statement whose expressions read a column, one per JSON constructor context.
const MAX_PROBES: usize = 8;
/// How many times one analysis may settle a probe at all, the same statement under another
/// assumption included.
const MAX_PROBE_CALLS: usize = 64;

// The catalog queries below are public so that a host with a driver of its own can bind their
// parameters by type: each casts every parameter in its text, and the doc comment names them.

/// `$1` is `oid[]` of relations, `$2` the `int2[]` of attribute numbers beside them.
pub const ATTNOTNULL_QUERY: &str = "SELECT a.attrelid::int8, a.attnum::int4 FROM pg_attribute a \
    JOIN unnest($1::oid[], $2::int2[]) AS u(rel, att) ON a.attrelid = u.rel AND a.attnum = u.att WHERE a.attnotnull";
/// `$1` is `oid[]` of relations.
pub const RELNAMES_QUERY: &str = "SELECT c.oid::int8, c.relname::text FROM pg_class c WHERE c.oid = ANY($1::oid[])";
/// Every column of the relations the parse names, by name, so provenance and `NOT NULL` are
/// settled without a second round for the tables the statement itself reads. `$1` is `text[]` of
/// relation names.
pub const RELATION_COLUMNS_QUERY: &str = "SELECT c.oid::int8, c.relname::text, a.attnum::int4, a.attnotnull, \
    CASE WHEN c.relkind IN ('v', 'm') AND a.attnum = 1 THEN pg_catalog.pg_get_viewdef(c.oid) END, a.attname::text \
    FROM pg_class c JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped \
    WHERE c.relname = ANY($1::text[]) AND pg_catalog.pg_table_is_visible(c.oid)";
/// Which of the names the expression rules read a user or an extension has defined, and which
/// built-in types a cast may target (see [`crate::infer::Trust`]). `$1` is `text[]` of operator
/// names, `$2` the `text[]` of function names.
pub const TRUST_QUERY: &str = "SELECT false, o.oprname::text FROM pg_operator o WHERE o.oid >= 16384 AND o.oprname::text = ANY($1::text[]) \
    UNION ALL SELECT false, p.proname::text FROM pg_proc p WHERE p.oid >= 16384 AND p.proname::text = ANY($2::text[]) \
    UNION ALL SELECT true, t.typname::text FROM pg_type t WHERE t.oid < 16384 AND t.typnamespace = 'pg_catalog'::regnamespace \
      AND NOT EXISTS (SELECT 1 FROM pg_type u WHERE u.oid >= 16384 AND u.typname = t.typname) \
      AND NOT EXISTS (SELECT 1 FROM pg_cast c WHERE c.oid >= 16384 AND c.casttarget = t.oid AND c.castmethod = 'f')";

pub fn analyze(sql: &str, dialect: Dialect, host: &Host) -> Lazy<Analysis> {
    let lexed = match lexed(sql, dialect) {
        Ok(l) => l,
        Err(bare) => return Some(bare),
    };
    if dialect == Dialect::Sqlite {
        return crate::sqlite::analyze(sql, &lexed, host);
    }
    if dialect == Dialect::Mysql {
        return crate::mysql::analyze(sql, &lexed, host);
    }
    let engine = Engine::new(host);
    engine.run(sql, &lexed)
}

/// The described statement alone, for a host that renders its own types and reads its own JSON
/// shapes and row counts (the framework's `describe.rs`): the parameter types in `$n` order and
/// each result column's type and nullability, settled the way [`analyze`] settles them. `sql` is
/// already in the server's `$n` form and is described as it is, and a parameter may be null,
/// because such a host binds whatever its caller sends. `Some(Err)` is the server's refusal of
/// the statement itself.
pub fn describe(sql: &str, host: &Host) -> Lazy<Result<DescribedStatement, DbError>> {
    let engine = Engine::new(host);
    let parsed = analyze::analyze(sql).ok();
    let trust = trust(host);
    let relations = relations_of(parsed.as_ref());
    engine.described(sql, parsed.as_ref(), &relations, trust.as_ref()).map(|r| r.map(|(_, described)| described))
}

/// Which of the names the expression rules read are the built-ins, as the catalog says: one
/// session-scoped query.
pub fn trust(host: &Host) -> Lazy<Trust> {
    let operators: Vec<Value> = infer::POSTGRES_OPERATORS.iter().map(|s| json!(s)).collect();
    let functions: Vec<Value> = infer::postgres_function_names().into_iter().map(|s| json!(s)).collect();
    let rows = host.query(TRUST_QUERY, vec![json!(operators), json!(functions)], Scope::Session)?;
    let mut trust = Trust { shadowed: HashSet::new(), cast_targets: Some(HashSet::new()), ..Default::default() };
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

/// The catalog the row-count proof reads for a statement that calls `functions` and reads
/// `relations`: which of those functions return a set, the unique keys of those relations, and
/// every operator a set-returning function implements.
pub fn catalog(host: &Host, functions: &[String], relations: &[String]) -> Lazy<Catalog> {
    let operators = set_returning_operators(host);
    let rows = catalog_rows(host, functions, &sorted(relations.to_vec()));
    let (Some(operators), Some(rows)) = (operators, rows) else { return None };
    Some(build_catalog(&operators, rows))
}

fn set_returning_operators(host: &Host) -> Lazy<HashSet<String>> {
    let rows = host.query(cardinality::SET_RETURNING_OPERATORS_QUERY, vec![], Scope::Session)?;
    Some(rows.unwrap_or_default().iter().filter_map(|r| r.first().and_then(as_str).map(String::from)).collect())
}

/// The rows the row-count proof's catalog is built from: the unique keys of `relations`, and
/// which of `functions` return a set.
fn catalog_rows(host: &Host, functions: &[String], relations: &[String]) -> Lazy<(Vec<Vec<Value>>, Vec<Vec<Value>>)> {
    let keys = if relations.is_empty() { Some(Ok(Vec::new())) } else { host.query(cardinality::UNIQUE_KEYS_QUERY, vec![json!(relations)], Scope::Statement) };
    let functions = if functions.is_empty() { Some(Ok(Vec::new())) } else { host.query(cardinality::FUNCTIONS_QUERY, vec![json!(functions)], Scope::Statement) };
    let (Some(keys), Some(functions)) = (keys, functions) else { return None };
    Some((keys.unwrap_or_default(), functions.unwrap_or_default()))
}

/// The statement's placeholders, or the bare analysis that reports why they cannot be read: a
/// lex error, or named and positional parameters mixed.
pub(crate) fn lexed(sql: &str, dialect: Dialect) -> Result<Lexed, Analysis> {
    let lexed = match lex::lex(sql, dialect) {
        Ok(l) => l,
        Err(e) => return Err(bare(sql, &Lexed { params: vec![], placeholders: vec![], directives: vec![], style: Style::None, opaque: vec![] }, dialect, vec![diag(e.message, e.offset, 1, 1)])),
    };
    if lexed.style == Style::Mixed {
        let first = lexed.placeholders.iter().find(|p| lex::is_positional(&p.name)).expect("a mixed statement has a positional placeholder");
        let d = diag("Named and positional parameters cannot be mixed in one statement.", first.start, first.end - first.start, 2);
        return Err(bare(sql, &lexed, dialect, vec![d]));
    }
    Ok(lexed)
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
        params: lexed.params.iter().map(|n| Param { name: n.clone(), ty: Type::Unknown, type_name: String::new(), nullable: false }).collect(),
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
    /// The parse-named relations' columns: relation oid → (name, attnum → NOT NULL).
    relations: RefCell<HashMap<u32, (String, HashMap<i32, bool>)>>,
    /// The same columns by relation name, in attnum order: column name → NOT NULL. An INSERT
    /// or an UPDATE names the columns it writes, or their positions.
    columns_by_name: RefCell<HashMap<String, BTreeMap<i32, (String, bool)>>>,
    /// The defining SELECT of each of those that is a view or a materialized view. The catalog
    /// holds no NOT NULL for a view's columns, so they are settled from this statement instead.
    views: RefCell<HashMap<u32, String>>,
    /// The WITH queries being settled, outermost first: the statement text of each, the columns
    /// taken as nullable for a read of the query from inside itself, and whether such a read
    /// happened. See `with_columns`.
    with_stack: RefCell<Vec<(String, HashSet<String>, bool)>>,
    /// The statements probed so far. The budget is on distinct statements, which is what costs
    /// a describe; settling one again under another assumption costs nothing.
    probed: RefCell<HashSet<String>>,
    /// The statements being probed right now, outermost first. A statement that comes to probe
    /// itself (a whole-row leaf reads as a whole row in its own leaf probe) proves nothing by it.
    probing: RefCell<Vec<String>>,
    /// Every `probe` call so far, whatever the statement: a bound on the work one analysis does.
    probe_calls: Cell<usize>,
    /// Per placeholder of the main statement, the cast a probe pins on it.
    casts: RefCell<Vec<Option<String>>>,
    budget: Cell<usize>,
}

impl<'a> Engine<'a> {
    fn new(host: &'a Host<'a>) -> Engine<'a> {
        Engine { host, registry: RefCell::new(TypeRegistry::with_builtins()), relations: RefCell::new(HashMap::new()), columns_by_name: RefCell::new(HashMap::new()), views: RefCell::new(HashMap::new()), with_stack: RefCell::new(Vec::new()), probed: RefCell::new(HashSet::new()), probing: RefCell::new(Vec::new()), probe_calls: Cell::new(0), casts: RefCell::new(Vec::new()), budget: Cell::new(MAX_PROBES) }
    }

    fn run(&self, sql: &str, lexed: &Lexed) -> Lazy<Analysis> {
        let mut diagnostics = Vec::new();
        let rewritten = lex::rewrite(sql, lexed, Target::Dollar);
        let parsed = match analyze::analyze(&rewritten.sql) {
            Ok(p) => Some(p),
            // A statement the engine does not type (a cursor, a DO block, DDL) says so on the
            // whole statement, rather than coming back as `unknown` with no word.
            Err(e @ analyze::AnalyzeError::UnsupportedStatement) => {
                diagnostics.push(diag(e.to_string(), 0, sql.trim_end().len(), 10));
                None
            }
            Err(_) => None,
        };
        // The first round asks for everything at once: the session facts, the describe, and
        // what the parse alone names — the relations' columns, column types and unique keys,
        // the functions, and every probe that carries no placeholder. Only what depends on the
        // describe (a type it reports that no named relation holds, a probe that pins a
        // parameter's type) waits for a second round.
        let relations = relations_of(parsed.as_ref());
        // A parameter's TypeScript type carries null only where the statement lets the caller
        // pass one (see `nullable_params`); every other parameter is a non-null value.
        let nullable = self.nullable_params(parsed.as_ref(), &relations);
        let trust = trust(self.host).zip(nullable.clone()).map(|(mut trust, nullable)| {
            trust.non_null_params = true;
            trust.nullable_params = nullable;
            trust
        });
        let operators = set_returning_operators(self.host);
        let described = self.described(&rewritten.sql, parsed.as_ref(), &relations, trust.as_ref());
        let functions: &[String] = parsed.as_ref().map(|p| p.functions.as_slice()).unwrap_or(&[]);
        let catalog_rows = catalog_rows(self.host, functions, &relations);
        for probe in &jsonshape::plan(&rewritten.sql).probes {
            self.prewarm(&probe.sql);
        }
        let (Some(described), Some(trust), Some(operators), Some(catalog_rows)) = (described, trust, operators, catalog_rows) else { return None };
        let (raw, described) = match described {
            Ok(d) => d,
            Err(e) => {
                diagnostics.push(db_diagnostic(sql, &rewritten, &e));
                return Some(bare(sql, lexed, Dialect::Postgres, diagnostics));
            }
        };
        // The catalog the row-count proof reads.
        let catalog = build_catalog(&operators, catalog_rows);

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
                let nullable = nullable.as_ref().is_some_and(|n| n.contains(&(i as i32 + 1)));
                Param { name: name.clone(), ty: registry.render(o), type_name: registry.name(o), nullable }
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
        // By the top statement, not `kind`: a SELECT over a data-modifying WITH is a mutation,
        // and its rows are still a SELECT's.
        if parsed.statement != "select" {
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

    /// Every column and every column type of the named relations, in one round, so `settle`
    /// and `types` have their answers before the describe is back.
    fn prefetch(&self, relations: &[String]) -> Lazy<()> {
        if relations.is_empty() {
            return Some(());
        }
        let columns = self.host.query(RELATION_COLUMNS_QUERY, vec![json!(relations)], Scope::Statement);
        let types = self.host.query(RELATION_TYPES_QUERY, vec![json!(relations)], Scope::Statement);
        let (Some(columns), Some(types)) = (columns, types) else { return None };
        let mut known = self.relations.borrow_mut();
        for row in columns.unwrap_or_default() {
            let (Some(oid), Some(name), Some(attnum), Some(not_null)) = (row.first().and_then(as_u32), row.get(1).and_then(as_str), row.get(2).and_then(as_u32), row.get(3).and_then(as_bool)) else { continue };
            known.entry(oid).or_insert_with(|| (name.to_string(), HashMap::new())).1.insert(attnum as i32, not_null);
            if let Some(column) = row.get(5).and_then(as_str) {
                self.columns_by_name.borrow_mut().entry(name.to_string()).or_default().insert(attnum as i32, (column.to_string(), not_null));
            }
            if let Some(definition) = row.get(4).and_then(as_str) {
                self.views.borrow_mut().insert(oid, definition.trim_end().trim_end_matches(';').to_string());
            }
        }
        self.registry.borrow_mut().insert_rows(&types.unwrap_or_default());
        Some(())
    }

    /// The parameters the caller may pass null for, by number: one the statement tests for null
    /// or coalesces (`:p IS NULL OR …`, `coalesce(:p, 50)`), and one that is the whole value
    /// written to a column the catalog lets hold null, wherever it is written (a parameter that
    /// also fills a NOT NULL column cannot be null). The rules take every other parameter as a
    /// non-null value, which is what its TypeScript type promises.
    fn nullable_params(&self, parsed: Option<&Parsed>, relations: &[String]) -> Lazy<HashSet<i32>> {
        let Some(parsed) = parsed else { return Some(HashSet::new()) };
        let mut out: HashSet<i32> = parsed.params.iter().filter(|(_, u)| u.null_guarded || u.coalesce).map(|(n, _)| *n).collect();
        if parsed.filled.is_empty() {
            return Some(out);
        }
        self.prefetch(relations)?;
        let columns = self.columns_by_name.borrow();
        let mut fills: HashMap<i32, bool> = HashMap::new();
        for (n, relation, column) in &parsed.filled {
            let Some(known) = columns.get(relation) else { continue };
            let not_null = match column {
                analyze::Filled::Name(name) => known.values().find(|(c, _)| c == name).map(|(_, nn)| *nn),
                analyze::Filled::Position(i) => known.values().nth(*i).map(|(_, nn)| *nn),
            };
            if let Some(not_null) = not_null {
                *fills.entry(*n).or_insert(true) &= !not_null;
            }
        }
        out.extend(fills.into_iter().filter(|(_, nullable)| *nullable).map(|(n, _)| n));
        Some(out)
    }

    /// Ask for a probe's describe ahead of the round that reads it, when nothing has to be
    /// pinned on it first: `probe` finds the answer waiting.
    fn prewarm(&self, sql: &str) {
        if lex::lex(sql, Dialect::Postgres).is_ok_and(|l| l.placeholders.is_empty()) {
            let _ = self.host.describe(sql);
        }
    }

    // -----------------------------------------------------------------------------------------
    // Describe, settle, refine
    // -----------------------------------------------------------------------------------------

    /// The describe of `sql` (the `$n` form) with its columns' nullability settled and refined.
    /// What it needs is asked in the same round as the parse-named relations' columns and types
    /// and every probe that carries no placeholder. `Some(Err)` is the server's refusal.
    fn described(&self, sql: &str, parsed: Option<&Parsed>, relations: &[String], trust: Option<&Trust>) -> Lazy<Result<(RawDescribe, DescribedStatement), DbError>> {
        let raw = self.host.describe(sql);
        let prefetched = self.prefetch(relations);
        if let Some((_, queries)) = infer::with_reads(sql) {
            for query in queries.values() {
                self.prewarm(&query.sql);
            }
        }
        if let Some(trust) = trust {
            let plan = infer::plan(sql, Dialect::Postgres, trust);
            if let Some(set) = &plan.set_op {
                self.prewarm(&set.left);
                self.prewarm(&set.right);
            }
            if let (Some(leaves), true) = (&plan.leaves, plan.reads_leaves()) {
                self.prewarm(leaves);
            }
        }
        let (Some(raw), Some(trust), Some(())) = (raw, trust, prefetched) else { return None };
        let raw = match raw {
            Ok(r) => r,
            Err(e) => return Some(Err(e)),
        };
        // Types for the parameters and the columns, and the casts probes pin on the parameters.
        let mut oids: Vec<u32> = raw.params.clone();
        oids.extend(raw.columns.iter().map(|c| c.oid));
        let types = self.types(&oids);
        // Nullability from provenance, then from the SQL.
        let settled = self.settle(sql, &raw, parsed, trust);
        let (Some(()), Some(mut described)) = (types, settled) else { return None };
        *self.casts.borrow_mut() = raw.params.iter().map(|o| self.registry.borrow().get(*o).map(|t| t.cast_name.clone()).filter(|c| !c.is_empty())).collect();
        self.refine(sql, &mut described, trust)?;
        Some(Ok((raw, described)))
    }

    /// A described statement's columns with their nullability from provenance: the catalog's
    /// `NOT NULL`, unless the column's table sits on the nullable side of an outer join, or the
    /// statement's shape hides which relation produced a column.
    fn settle(&self, sql: &str, raw: &RawDescribe, parsed: Option<&Parsed>, trust: &Trust) -> Lazy<DescribedStatement> {
        // A column of a view: the view's own statement is settled like any other (its columns
        // trace to the base tables, its joins null a side, its expressions follow the rules),
        // and column `attnum` of the view is column `attnum - 1` of that result.
        let mut view_columns: HashMap<u32, Option<Vec<DescribedColumn>>> = HashMap::new();
        let definitions: Vec<(u32, String)> = {
            let views = self.views.borrow();
            let mut oids: Vec<u32> = raw.columns.iter().map(|c| c.table).filter(|t| views.contains_key(t)).collect();
            oids.sort_unstable();
            oids.dedup();
            oids.into_iter().map(|o| (o, views[&o].clone())).collect()
        };
        let mut pending = false;
        for (oid, definition) in definitions {
            match self.probe(&definition, trust) {
                Some(outcome) => {
                    view_columns.insert(oid, outcome.map(|d| d.columns));
                }
                None => pending = true,
            }
        }
        if pending {
            return None;
        }
        // The parse-named relations were prefetched; a column from any other relation (a view's
        // base table, a relation the parse could not name) is asked for by oid.
        let known = self.relations.borrow();
        let pairs: Vec<(u32, i32)> = raw.columns.iter().filter(|c| c.table != 0 && c.attnum > 0 && !known.contains_key(&c.table)).map(|c| (c.table, c.attnum)).collect();
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
        let mut not_null: HashSet<(u32, i32)> = not_null
            .unwrap_or_default()
            .iter()
            .filter_map(|r| Some((as_u32(r.first()?)?, as_u32(r.get(1)?)? as i32)))
            .collect();
        let mut nulled_rels: HashSet<u32> = relnames
            .unwrap_or_default()
            .iter()
            .filter_map(|r| {
                let (o, name) = (as_u32(r.first()?)?, as_str(r.get(1)?)?);
                nullable_tables.iter().any(|t| t == name).then_some(o)
            })
            .collect();
        for col in &raw.columns {
            if let Some((name, atts)) = known.get(&col.table) {
                if atts.get(&col.attnum).copied().unwrap_or(false) {
                    not_null.insert((col.table, col.attnum));
                }
                if nullable_tables.iter().any(|t| t == name) {
                    nulled_rels.insert(col.table);
                }
            }
        }
        // Without a parse tree nothing degrades and nothing is nulled by a join, which is
        // wrong in the unsafe direction, so a statement libpg_query cannot read keeps every
        // column nullable.
        let hidden = match parsed {
            Some(p) => (p.has_repeated_table && !p.nullable_tables.is_empty()) || p.has_grouping_sets,
            None => true,
        };
        // A set operation anywhere: provenance names the leftmost arm's column, which says
        // nothing about the other arms. The statement's own columns stay nullable; a column read
        // from a WITH query is settled from that query's own probe, whose arms are combined.
        let degraded = hidden || parsed.is_some_and(|p| p.has_set_op);
        // A column read straight from a WITH query is as nullable as that query's own result
        // says, whatever the catalog says of the table column behind it.
        let with_columns = if hidden { Some(Vec::new()) } else { self.with_columns(sql, raw.columns.len(), trust) };
        let Some(with_columns) = with_columns else { return None };
        let registry = self.registry.borrow();
        let columns: Vec<DescribedColumn> = raw
            .columns
            .iter()
            .map(|col| {
                let nullable = if col.table != 0 && col.attnum > 0 && !degraded {
                    let own = match view_columns.get(&col.table) {
                        Some(columns) => columns.as_ref().and_then(|c| c.get(col.attnum as usize - 1)).is_none_or(|c| c.nullable),
                        None => !not_null.contains(&(col.table, col.attnum)),
                    };
                    own || nulled_rels.contains(&col.table)
                } else {
                    true
                };
                DescribedColumn { name: col.name.clone(), type_oid: col.oid, type_name: registry.name(col.oid), nullable }
            })
            .collect();
        let columns = columns
            .into_iter()
            .enumerate()
            .map(|(i, col)| match with_columns.get(i) {
                Some(Some((query, nullable))) => DescribedColumn { nullable: *nullable || nullable_tables.iter().any(|t| t == query), ..col },
                _ => col,
            })
            .collect();
        Some(DescribedStatement { param_oids: raw.params.clone(), columns })
    }

    /// Per result column of `sql` that is a bare reference to a column of one of its WITH
    /// queries: that query's name, and whether its own settled result has the column nullable.
    ///
    /// A recursive WITH query reads itself, so its columns' nullability is a fixed point, and
    /// the greatest one is sound. Every column starts as non-null for the reads the query makes
    /// of itself; the query is settled under that assumption; a column that comes out nullable
    /// all the same joins the assumption as nullable, and the query is settled again, until
    /// nothing changes. What survives holds by induction over the iterations: the rows of the
    /// non-recursive arm are non-null on their own account, each later row is non-null given
    /// the rows it was computed from, and a UNION is non-null only where both arms are.
    fn with_columns(&self, sql: &str, count: usize, trust: &Trust) -> Lazy<Vec<Option<(String, bool)>>> {
        let Some((reads, queries)) = infer::with_reads(sql) else { return Some(Vec::new()) };
        if reads.len() != count {
            return Some(Vec::new());
        }
        let mut settled: HashMap<String, Vec<(String, bool)>> = HashMap::new();
        let mut pending = false;
        for (name, query) in &queries {
            // The query reading itself: the answer is the assumption it is being settled under.
            if let Some(entry) = self.with_stack.borrow_mut().iter_mut().find(|(text, _, _)| *text == query.sql) {
                entry.2 = true;
                let assumed = &entry.1;
                settled.insert(name.clone(), reads.iter().flatten().filter(|r| &r.query == name).map(|r| (r.column.clone(), assumed.contains(&r.column))).collect());
                continue;
            }
            let mut assumed: HashSet<String> = HashSet::new();
            loop {
                self.with_stack.borrow_mut().push((query.sql.clone(), assumed.clone(), false));
                let outcome = self.probe(&query.sql, trust);
                let (_, _, read_itself) = self.with_stack.borrow_mut().pop().expect("the entry pushed above");
                let described = match outcome {
                    Some(Some(described)) => described,
                    // A WITH query the server will not describe alone proves nothing.
                    Some(None) => break,
                    None => {
                        pending = true;
                        break;
                    }
                };
                let columns: Vec<(String, bool)> = described.columns.iter().enumerate().map(|(i, c)| (query.columns.get(i).cloned().unwrap_or_else(|| c.name.clone()), c.nullable)).collect();
                let refuted: Vec<String> = columns.iter().filter(|(n, nullable)| *nullable && !assumed.contains(n)).map(|(n, _)| n.clone()).collect();
                if !read_itself || refuted.is_empty() {
                    settled.insert(name.clone(), columns);
                    break;
                }
                assumed.extend(refuted);
            }
        }
        if pending {
            return None;
        }
        Some(
            reads
                .into_iter()
                .map(|read| {
                    let read = read?;
                    let mut named = settled.get(&read.query)?.iter().filter(|(n, _)| *n == read.column);
                    match (named.next(), named.next()) {
                        (Some((_, nullable)), None) => Some((read.query, *nullable)),
                        _ => None,
                    }
                })
                .collect(),
        )
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
        if self.probing.borrow().iter().any(|p| p == sql) || self.probe_calls.get() >= MAX_PROBE_CALLS {
            return Some(None);
        }
        self.probe_calls.set(self.probe_calls.get() + 1);
        if !self.probed.borrow().contains(sql) {
            if self.budget.get() == 0 {
                return Some(None);
            }
            self.budget.set(self.budget.get() - 1);
            self.probed.borrow_mut().insert(sql.to_string());
        }
        self.probing.borrow_mut().push(sql.to_string());
        let outcome = self.probe_inner(sql, trust);
        self.probing.borrow_mut().pop();
        outcome
    }

    fn probe_inner(&self, sql: &str, trust: &Trust) -> Lazy<Option<DescribedStatement>> {
        let pinned = self.pin(sql);
        let raw = self.host.describe(&pinned)?;
        let Ok(raw) = raw else { return Some(None) };
        let parsed = analyze::analyze(sql).ok();
        let mut oids: Vec<u32> = raw.columns.iter().map(|c| c.oid).collect();
        oids.extend(raw.params.iter().copied());
        let types = self.types(&oids);
        let settled = self.settle(sql, &raw, parsed.as_ref(), trust);
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
            let cast = name.parse::<usize>().ok().and_then(|n| casts.get(n.wrapping_sub(1)).cloned().flatten());
            match cast {
                Some(c) => format!("({rendered}::{c})"),
                None => rendered.to_string(),
            }
        })
        .sql
    }
}

/// The relations a statement reads or writes, as the parse names them, sorted. libpg_query
/// collects them through a hash set, so each parse lists them in another order; a query that
/// carried that order would be a new need every round, and the replay would settle only when two
/// rounds happened to agree.
fn relations_of(parsed: Option<&Parsed>) -> Vec<String> {
    let Some(parsed) = parsed else { return Vec::new() };
    sorted(parsed.read_set.iter().chain(&parsed.write_set).cloned().collect())
}

fn sorted(mut names: Vec<String>) -> Vec<String> {
    names.sort_unstable();
    names.dedup();
    names
}

fn build_catalog(operators: &HashSet<String>, rows: (Vec<Vec<Value>>, Vec<Vec<Value>>)) -> Catalog {
    let mut catalog = Catalog { set_returning_operators: operators.clone(), ..Catalog::default() };
    for row in rows.0 {
        if let (Some(rel), Some(cols)) = (row.first().and_then(as_str), row.get(1).and_then(as_strings)) {
            catalog.unique_keys.entry(rel.to_string()).or_default().push(cols);
        }
    }
    for row in rows.1 {
        if let (Some(name), Some(retset)) = (row.first().and_then(as_str), row.get(1).and_then(as_bool)) {
            if retset {
                catalog.set_returning.insert(name.to_string());
            }
            catalog.functions.insert(name.to_string());
        }
    }
    catalog
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Answer, RawColumn, RawDescribe};

    /// A server that describes every statement as one text column per target and answers every
    /// catalog query with no rows: enough for the replay to run to its end.
    /// How many result columns the top statement's own target list names (the leftmost arm's
    /// for a set operation), which is what the fake describer reports.
    fn target_count(sql: &str) -> usize {
        fn arm(s: &pg_query::protobuf::SelectStmt) -> usize {
            match s.larg.as_deref() {
                Some(left) => arm(left),
                None => s.target_list.len(),
            }
        }
        let parsed = pg_query::parse(sql).expect("the probe parses");
        match parsed.protobuf.stmts.first().and_then(|s| s.stmt.as_deref()).and_then(|n| n.node.as_ref()) {
            Some(pg_query::NodeEnum::SelectStmt(s)) => arm(s).max(1),
            _ => 1,
        }
    }

    fn replay(sql: &str) -> Analysis {
        let mut answers: HashMap<String, Answer> = HashMap::new();
        for _ in 0..64 {
            let host = Host::new(&answers);
            if let Some(analysis) = analyze(sql, Dialect::Postgres, &host) {
                return analysis;
            }
            let needs = host.needs();
            assert!(!needs.is_empty(), "no answer and no need");
            for need in needs {
                match need {
                    crate::api::Need::Describe { id, sql } => {
                        let columns = (0..target_count(&sql)).map(|i| RawColumn { name: format!("c{i}"), oid: 25, table: 0, attnum: 0 }).collect();
                        answers.insert(id, Answer::Describe { describe: RawDescribe { params: vec![], columns } });
                    }
                    crate::api::Need::Query { id, .. } => {
                        answers.insert(id, Answer::Rows { rows: vec![] });
                    }
                }
            }
        }
        panic!("the replay did not converge");
    }

    #[test]
    fn a_probe_that_reads_as_its_own_leaf_ends() {
        // `u` is a whole-row leaf, and in its leaf probe `SELECT u AS a0 FROM users u` it is one
        // again: without a guard the probe would probe itself without end.
        let analysis = replay("SELECT u AS whole FROM users u");
        assert_eq!(analysis.columns.len(), 1);
    }

    #[test]
    fn a_round_names_the_same_needs_however_the_parse_orders_its_tables() {
        // libpg_query lists a statement's tables in hash order, which differs between parses; a
        // need that carried that order would be new every round and the replay would not settle.
        let sql = "SELECT a.x FROM alpha a JOIN beta b ON b.id = a.id JOIN gamma g ON g.id = b.id JOIN delta d ON d.id = g.id";
        let answers: HashMap<String, Answer> = HashMap::new();
        let round = || {
            let host = Host::new(&answers);
            assert!(analyze(sql, Dialect::Postgres, &host).is_none());
            host.needs()
        };
        let first = round();
        for _ in 0..16 {
            assert_eq!(round(), first);
        }
    }

    #[test]
    fn a_recursive_with_query_is_settled_from_its_own_arms() {
        // The UNION ALL sits inside the WITH query, so the statement's provenance is not to be
        // trusted, but the column read from the query takes what the query's own probe proves:
        // `1` is never null, and `n + 1` is not once `n < 5` holds.
        let seq = replay("WITH RECURSIVE seq(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM seq WHERE n < 5) SELECT n FROM seq");
        assert_eq!(seq.columns.iter().map(|c| c.nullable).collect::<Vec<_>>(), vec![false]);
        // `b` starts NULL and `a` takes it over: the fixed point proves neither.
        let pair = replay("WITH RECURSIVE pair(a, b) AS (SELECT 1, NULL::int UNION ALL SELECT b, b FROM pair WHERE a < 3) SELECT a, b FROM pair");
        assert_eq!(pair.columns.iter().map(|c| c.nullable).collect::<Vec<_>>(), vec![true, true]);
    }
}
