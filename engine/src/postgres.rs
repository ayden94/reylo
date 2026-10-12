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
use crate::levels::Columns;
use crate::lex::{self, Lexed, Rewritten, Style, Target};
use crate::pgtypes::{not_null_rows, TypeRegistry, RELATION_TYPES_QUERY, TYPE_QUERY};
use crate::returning::Rows;
use crate::types::{DescribedColumn, DescribedStatement, Type};
use pg_query::protobuf::node::Node as NodeEnum;
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
pub const ATTNOTNULL_QUERY: &str = concat!(
    "SELECT a.attrelid::int8, a.attnum::int4 FROM pg_attribute a \
    JOIN unnest($1::oid[], $2::int2[]) AS u(rel, att) ON a.attrelid = u.rel AND a.attnum = u.att WHERE a.attnotnull AND ",
    not_null_rows!("a.attrelid", "a.attname")
);
/// `$1` is `oid[]` of relations, `$2` the `int2[]` of attribute numbers beside them: one row per
/// validated, enforced CHECK on such a column, the table's on that column alone (and inherited by
/// any child a scan reads) or the column's domain's, where the column holds `text` or `varchar`
/// under a deterministic collation, `(relation, attnum, the column's name, the CHECK as the server
/// prints it)`. `conenforced` is read through `to_jsonb`, since only PostgreSQL 18 has the column.
/// A table's CHECK says nothing where the table, or a partition or child a scan reads, is a foreign
/// table, whose rows the server never checks; a domain's input function checks every value.
pub const CHECKS_QUERY: &str = "SELECT a.attrelid::int8, a.attnum::int4, a.attname::text, pg_catalog.pg_get_expr(k.conbin, k.conrelid, true) \
    FROM unnest($1::oid[], $2::int2[]) AS u(rel, att) \
    JOIN pg_attribute a ON a.attrelid = u.rel AND a.attnum = u.att AND NOT a.attisdropped \
    JOIN pg_type t ON t.oid = a.atttypid \
    JOIN pg_constraint k ON k.contype = 'c' AND k.convalidated AND coalesce((pg_catalog.to_jsonb(k) ->> 'conenforced')::bool, true) \
      AND ((k.conrelid = a.attrelid AND k.conkey = ARRAY[a.attnum]::int2[] \
          AND (NOT k.connoinherit OR NOT EXISTS (SELECT 1 FROM pg_inherits h WHERE h.inhparent = a.attrelid)) \
          AND NOT EXISTS (WITH RECURSIVE tree(rel) AS (SELECT a.attrelid UNION SELECT i.inhrelid FROM tree t JOIN pg_inherits i ON i.inhparent = t.rel) \
            SELECT 1 FROM tree t JOIN pg_class f ON f.oid = t.rel WHERE f.relkind = 'f')) \
        OR (t.typtype = 'd' AND k.contypid = t.oid)) \
    WHERE (CASE WHEN t.typtype = 'd' THEN t.typbasetype ELSE t.oid END) IN ('pg_catalog.text'::pg_catalog.regtype, 'pg_catalog.varchar'::pg_catalog.regtype) \
      AND coalesce((SELECT o.collisdeterministic FROM pg_collation o WHERE o.oid = a.attcollation), true)";
/// `$1` is `oid[]` of relations.
pub const RELNAMES_QUERY: &str = "SELECT c.oid::int8, c.relname::text FROM pg_class c WHERE c.oid = ANY($1::oid[])";
/// Every column of the relations the parse names, by name, so provenance and `NOT NULL` are
/// settled without a second round for the tables the statement itself reads. `$1` is `text[]` of
/// relation names. The fourth column is the `NOT NULL` a write meets, which checks every new row
/// whether or not the constraint is valid; the seventh is the one a read may trust (see
/// `not_null_rows`).
pub const RELATION_COLUMNS_QUERY: &str = concat!(
    "SELECT c.oid::int8, c.relname::text, a.attnum::int4, a.attnotnull, \
    CASE WHEN c.relkind IN ('v', 'm') AND a.attnum = 1 THEN pg_catalog.pg_get_viewdef(c.oid) END, a.attname::text, \
    a.attnotnull AND ",
    not_null_rows!("c.oid", "a.attname"),
    " FROM pg_class c JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped \
    WHERE c.relname = ANY($1::text[]) AND pg_catalog.pg_table_is_visible(c.oid)"
);
/// Which of the names the expression rules read a user or an extension has defined (`false`
/// beside the name), and which built-in types a cast may target (`true`; see
/// [`crate::infer::Trust`]). `$1` is `text[]` of operator names, `$2` the `text[]` of function
/// names. One more row, a NULL beside [`JSONB_CASTS_RAISE`], comes from a server before 18, whose
/// cast of a JSON null to a number or a boolean raises an error where 18 answers NULL.
pub const TRUST_QUERY: &str = "SELECT false, o.oprname::text FROM pg_operator o WHERE o.oid >= 16384 AND o.oprname::text = ANY($1::text[]) \
    UNION ALL SELECT false, p.proname::text FROM pg_proc p WHERE p.oid >= 16384 AND p.proname::text = ANY($2::text[]) \
    UNION ALL SELECT true, t.typname::text FROM pg_type t WHERE t.oid < 16384 AND t.typnamespace = 'pg_catalog'::regnamespace \
      AND NOT EXISTS (SELECT 1 FROM pg_type u WHERE u.oid >= 16384 AND u.typname = t.typname) \
      AND NOT EXISTS (SELECT 1 FROM pg_cast c WHERE c.oid >= 16384 AND c.casttarget = t.oid AND c.castmethod = 'f') \
    UNION ALL SELECT NULL, 'jsonb casts raise' WHERE pg_catalog.current_setting('server_version_num')::int4 < 180000";
/// The name of [`TRUST_QUERY`]'s row that says the server is one before PostgreSQL 18.
const JSONB_CASTS_RAISE: &str = "jsonb casts raise";

/// `$1` is `text[]` of relation names as the statement writes them: one row per unique index of
/// each that is a plain or a partitioned table, `(the name, the index's key columns with NULL for
/// a key part that is an expression, the index's predicate as the server prints it)`, and for such
/// a table with no unique index one row of the name beside NULLs. These are the indexes an `ON
/// CONFLICT (…)` may infer as its arbiter (the planner's `infer_arbiter_indexes`), and more: an
/// invalid one is listed too, so a missing row is a missing index. The index of a key `WITHOUT
/// OVERLAPS` (PostgreSQL 18) is unique by name and an exclusion constraint in fact, which the
/// planner infers for no column list, so it is none of them. A view, whose write goes to another
/// relation, has no row.
pub const ARBITERS_QUERY: &str = concat!(
    "SELECT x.name, \
      (SELECT array_agg(a.attname::text ORDER BY k.ord) FROM unnest(i.indkey::int2[]) WITH ORDINALITY AS k(attnum, ord) \
        LEFT JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = k.attnum AND k.attnum > 0 WHERE k.ord <= i.indnkeyatts), \
      pg_catalog.pg_get_expr(i.indpred, i.indrelid, true) \
    FROM unnest($1::text[]) AS x(name) \
    JOIN pg_class c ON c.oid = ",
    cardinality::relation_named!("x.name"),
    " AND c.relkind IN ('r', 'p') \
    LEFT JOIN pg_index i ON i.indrelid = c.oid AND i.indisunique AND NOT i.indisexclusion"
);

pub fn analyze(sql: &str, dialect: Dialect, host: &Host) -> Lazy<Analysis> {
    host.on_engine_stack(|host| analyze_here(sql, dialect, host))
}

fn analyze_here(sql: &str, dialect: Dialect, host: &Host) -> Lazy<Analysis> {
    let lexed = match lexed(sql, dialect) {
        Ok(l) => l,
        Err(bare) => return Some(bare),
    };
    if dialect == Dialect::Sqlite {
        return crate::sqlite::analyze(sql, &lexed, host);
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
    host.on_engine_stack(|host| {
        let engine = Engine::new(host);
        let parsed = analyze::analyze(sql).ok();
        let trust = trust(host);
        let relations = relations_of(parsed.as_ref());
        engine.described(sql, parsed.as_ref(), &relations, trust.as_ref()).map(|r| r.map(|(_, described)| described))
    })
}

/// Which of the names the expression rules read are the built-ins, as the catalog says: one
/// session-scoped query.
pub fn trust(host: &Host) -> Lazy<Trust> {
    let mut operators: Vec<&str> = infer::POSTGRES_OPERATORS.to_vec();
    operators.extend(infer::POSTGRES_STRICT_OPERATORS.iter().filter(|o| !infer::POSTGRES_OPERATORS.contains(o)));
    let operators: Vec<Value> = operators.into_iter().map(|s| json!(s)).collect();
    let functions: Vec<Value> = infer::postgres_function_names().into_iter().map(|s| json!(s)).collect();
    let rows = host.query(TRUST_QUERY, vec![json!(operators), json!(functions)], Scope::Session)?;
    let mut trust = Trust { shadowed: HashSet::new(), cast_targets: Some(HashSet::new()), ..Default::default() };
    if let Ok(rows) = rows {
        for row in rows {
            let Some(name) = row.get(1).and_then(as_str) else { continue };
            match row.first().and_then(as_bool) {
                Some(true) => {
                    trust.cast_targets.get_or_insert_with(HashSet::new).insert(name.to_string());
                }
                Some(false) => {
                    trust.shadowed.insert(name.to_string());
                }
                None => trust.jsonb_casts_raise |= name == JSONB_CASTS_RAISE,
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
    let trust = trust(host);
    let (Some(operators), Some(rows), Some(trust)) = (operators, rows, trust) else { return None };
    let mut catalog = build_catalog(&operators, rows);
    trusted(&mut catalog, &trust);
    Some(catalog)
}

/// The columns of the relations `sql` reads, for a statement with a scalar subquery, where an
/// aggregate is the subquery's own only if a column it reads is one of the subquery's FROM
/// ([`crate::levels`]); no columns, and no question to the host, for any other statement. `sql`
/// is in the server's `$n` form. For the main statement this is the question the row-count proof
/// asks of the same relations, so it costs no query of its own.
fn level_columns(sql: &str, host: &Host) -> Lazy<Columns> {
    columns_where(crate::levels::needed(sql), sql, host)
}

/// The columns [`jsonshape::plan`] reads for `sql`, for a host that plans the shapes itself: none,
/// and no question, for a statement whose shapes do not turn on them.
pub fn shape_columns(sql: &str, host: &Host) -> Lazy<Columns> {
    host.on_engine_stack(|host| columns_where(jsonshape::needs_columns(sql), sql, host))
}

fn columns_where(needed: bool, sql: &str, host: &Host) -> Lazy<Columns> {
    if !needed {
        return Some(Columns::default());
    }
    let relations = relations_of(analyze::analyze(sql).ok().as_ref());
    if relations.is_empty() {
        return Some(Columns::default());
    }
    let rows = host.query(cardinality::COLUMNS_QUERY, vec![json!(relations)], Scope::Statement)?;
    Some(Columns::from_rows(&rows.unwrap_or_default()))
}

/// What the trust check says that the row-count proof reads: whether `=` is only the built-in
/// one, which operators a user defined, and which types no user's cast function targets.
fn trusted(catalog: &mut Catalog, trust: &Trust) {
    catalog.equality_shadowed = !trust.builtin("=");
    catalog.user_operators = trust.shadowed.clone();
    catalog.cast_targets = trust.cast_targets.clone();
}

fn set_returning_operators(host: &Host) -> Lazy<HashSet<String>> {
    let rows = host.query(cardinality::SET_RETURNING_OPERATORS_QUERY, vec![], Scope::Session)?;
    Some(rows.unwrap_or_default().iter().filter_map(|r| r.first().and_then(as_str).map(String::from)).collect())
}

/// The rows the row-count proof's catalog is built from: the unique keys, columns and skipping
/// triggers of `relations`, and which of `functions` return a set or are volatile.
fn catalog_rows(host: &Host, functions: &[String], relations: &[String]) -> Lazy<CatalogRows> {
    let none = || Some(Ok(Vec::new()));
    let keys = if relations.is_empty() { none() } else { host.query(cardinality::UNIQUE_KEYS_QUERY, vec![json!(relations)], Scope::Statement) };
    let columns = if relations.is_empty() { none() } else { host.query(cardinality::COLUMNS_QUERY, vec![json!(relations)], Scope::Statement) };
    let functions = if functions.is_empty() { none() } else { host.query(cardinality::FUNCTIONS_QUERY, vec![json!(functions)], Scope::Statement) };
    let triggers = if relations.is_empty() { none() } else { host.query(cardinality::TRIGGERS_QUERY, vec![json!(relations)], Scope::Statement) };
    let (Some(keys), Some(columns), Some(functions), Some(triggers)) = (keys, columns, functions, triggers) else { return None };
    Some(CatalogRows {
        keys: keys.unwrap_or_default(),
        columns: columns.unwrap_or_default(),
        functions: functions.unwrap_or_default(),
        triggers: triggers.unwrap_or_default(),
    })
}

/// The rows of the row-count proof's catalog queries, as the host answered them.
struct CatalogRows {
    keys: Vec<Vec<Value>>,
    columns: Vec<Vec<Value>>,
    functions: Vec<Vec<Value>>,
    triggers: Vec<Vec<Value>>,
}

/// The statement's placeholders, or the bare analysis that reports why they cannot be read: a
/// lex error, named and positional parameters mixed, or a parameter used more than once.
pub(crate) fn lexed(sql: &str, dialect: Dialect) -> Result<Lexed, Analysis> {
    let lexed = match lex::lex(sql, dialect) {
        Ok(l) => l,
        Err(e) => return Err(bare(sql, &Lexed { params: vec![], placeholders: vec![], style: Style::None, opaque: vec![] }, dialect, vec![diag(e.message, e.offset, 1, 1)])),
    };
    if lexed.style == Style::Mixed {
        let first = lexed.placeholders.iter().find(|p| lex::is_positional(&p.name)).expect("a mixed statement has a positional placeholder");
        let d = diag("Named and positional parameters cannot be mixed in one statement.", first.start, first.end - first.start, 2);
        return Err(bare(sql, &lexed, dialect, vec![d]));
    }
    let repeated = repeated(sql, &lexed);
    if !repeated.is_empty() {
        return Err(bare(sql, &lexed, dialect, repeated));
    }
    Ok(lexed)
}

/// Each later use of a parameter, as the diagnostic at its place ([`lex::repeated`]).
pub(crate) fn repeated(sql: &str, lexed: &Lexed) -> Vec<Diagnostic> {
    lex::repeated(sql, lexed).into_iter().map(|r| diag(r.message, r.start, r.end - r.start, 3)).collect()
}

fn diag(message: impl Into<String>, start: usize, length: usize, code: u32) -> Diagnostic {
    Diagnostic { message: message.into(), start, length: length.max(1), code }
}

/// What the engine answers when it cannot describe: every parameter unknown, no columns, and
/// `many` rows.
pub(crate) fn bare(sql: &str, lexed: &Lexed, dialect: Dialect, diagnostics: Vec<Diagnostic>) -> Analysis {
    let target = if dialect == Dialect::Postgres { Target::Dollar } else { Target::Question };
    let rewritten = lex::rewrite(sql, lexed, target);
    Analysis {
        kind: String::new(),
        params: lexed.params.iter().map(|n| Param { name: n.clone(), ty: Type::Unknown, type_name: String::new(), nullable: false }).collect(),
        columns: Vec::new(),
        cardinality: "many".to_string(),
        cardinality_reason: None,
        rewritten,
        style: lexed.style,
        diagnostics,
        unresolved: Vec::new(),
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
        for probe in &jsonshape::probes_ahead(&rewritten.sql) {
            self.prewarm(&probe.sql);
        }
        // The unique indexes of every table an `ON CONFLICT (…)` names its arbiter's columns for.
        let conflicts = conflict_targets(&rewritten.sql);
        let arbiters = if conflicts.is_empty() {
            Some(Ok(Vec::new()))
        } else {
            self.host.query(ARBITERS_QUERY, vec![json!(sorted(conflicts.iter().map(|c| c.relation.clone()).collect()))], Scope::Statement)
        };
        let (Some(described), Some(trust), Some(operators), Some(catalog_rows), Some(arbiters)) = (described, trust, operators, catalog_rows, arbiters) else { return None };
        let (raw, described) = match described {
            Ok(d) => d,
            Err(e) => {
                diagnostics.push(db_diagnostic(sql, &rewritten, &e));
                return Some(bare(sql, lexed, Dialect::Postgres, diagnostics));
            }
        };
        // The catalog the row-count proof reads, with what the describe and the trust settled.
        let mut catalog = build_catalog(&operators, catalog_rows);
        catalog.param_types = raw.params.clone();
        trusted(&mut catalog, &trust);
        let arbiters = arbiters.unwrap_or_default();
        for target in &conflicts {
            if let Some(message) = unmatched_conflict(target, &arbiters, &catalog) {
                let start = rewritten.to_original(target.start).min(sql.len());
                let length = sql[start..].find(')').map_or(1, |close| close + 1);
                diagnostics.push(diag(message, start, length, 13));
            }
        }

        // JSON shapes.
        let plan = jsonshape::plan(&rewritten.sql, Dialect::Postgres, &trust, &catalog.columns);
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
                _ => (with_values(with_items(registry.render(col.type_oid), col.nullable_items), col.values.as_deref()), col.nullable),
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
        let (cardinality, reason) = self.cardinality(&rewritten.sql, parsed.as_ref(), &described, &trust, &catalog);

        Some(Analysis {
            kind: parsed.as_ref().map(|p| p.statement.to_string()).unwrap_or_default(),
            params,
            columns,
            cardinality,
            cardinality_reason: reason,
            rewritten,
            style: lexed.style,
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
            let (Some(oid), Some(name), Some(attnum), Some(written)) = (row.first().and_then(as_u32), row.get(1).and_then(as_str), row.get(2).and_then(as_u32), row.get(3).and_then(as_bool)) else { continue };
            // A read trusts only a valid NOT NULL that holds through the inheritance tree; a
            // write meets the constraint as it stands, valid or not.
            let read = row.get(6).and_then(as_bool).unwrap_or(false);
            known.entry(oid).or_insert_with(|| (name.to_string(), HashMap::new())).1.insert(attnum as i32, read);
            if let Some(column) = row.get(5).and_then(as_str) {
                self.columns_by_name.borrow_mut().entry(name.to_string()).or_default().insert(attnum as i32, (column.to_string(), written));
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
        if let Some((_, queries)) = infer::item_reads(sql) {
            for query in &queries {
                self.prewarm(&query.sql);
            }
        }
        // The columns that say whose an aggregate in a scalar subquery is, which `refine` reads:
        // no probe waits on them.
        let columns = level_columns(sql, self.host).unwrap_or_default();
        if let Some(trust) = trust {
            let plan = infer::plan(sql, Dialect::Postgres, trust, &columns);
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
        let checks = self.checks(&raw, parsed, trust);
        // Nullability from provenance, then from the SQL.
        let settled = self.settle(sql, &raw, parsed, trust);
        let (Some(()), Some(mut described), Some(checks)) = (types, settled, checks) else { return None };
        for (col, raw) in described.columns.iter_mut().zip(&raw.columns) {
            col.values = checks.get(&(raw.table, raw.attnum)).cloned();
        }
        *self.casts.borrow_mut() = raw.params.iter().map(|o| self.registry.borrow().get(*o).map(|t| t.cast_name.clone()).filter(|c| !c.is_empty())).collect();
        self.refine(sql, &mut described, trust)?;
        self.without_missing_rows(sql, parsed, &raw, &mut described);
        Some(Ok((raw, described)))
    }

    /// PostgreSQL 18's `RETURNING old.*` and `new.*`, and a MERGE's source where the MERGE acts
    /// without one, read a row that may not exist, whatever the provenance the describe reports
    /// for them says: such a column is nullable. A probe is settled this way too, since it carries
    /// the statement's WITH queries, and a data-modifying one may return such a row.
    fn without_missing_rows(&self, sql: &str, parsed: Option<&Parsed>, raw: &RawDescribe, described: &mut DescribedStatement) {
        if parsed.is_none_or(|p| p.kind != analyze::StatementKind::Mutation) {
            return;
        }
        let known = self.columns_by_name.borrow();
        let columns = |relation: &str| known.get(relation).map(|columns| columns.values().map(|(name, _)| name.clone()).collect::<Vec<_>>());
        let origins: Vec<bool> = raw.columns.iter().map(|c| c.table != 0).collect();
        for (col, missing) in described.columns.iter_mut().zip(missing_rows(sql, &origins, &columns)) {
            if missing {
                col.nullable = true;
            }
        }
    }

    /// Per result column that reads a table's column as it is, the strings a CHECK keeps it to
    /// ([`check_values`]). The describe names a column's table and attribute only for a bare
    /// reference (through a view or a subquery too), so an expression over the column is never
    /// one. Nothing under a set operation, whose provenance names the leftmost arm only, and
    /// nothing where a user's `=` may be the one a CHECK resolved to.
    fn checks(&self, raw: &RawDescribe, parsed: Option<&Parsed>, trust: &Trust) -> Lazy<HashMap<(u32, i32), Vec<String>>> {
        let usable = parsed.is_some_and(|p| !p.has_set_op) && trust.builtin("=");
        let pairs: Vec<(u32, i32)> = raw.columns.iter().filter(|c| usable && c.table != 0 && c.attnum > 0).map(|c| (c.table, c.attnum)).collect();
        if pairs.is_empty() {
            return Some(HashMap::new());
        }
        let rs: Vec<u32> = pairs.iter().map(|(r, _)| *r).collect();
        let atts: Vec<i32> = pairs.iter().map(|(_, a)| *a).collect();
        let rows = self.host.query(CHECKS_QUERY, vec![json!(rs), json!(atts)], Scope::Statement)?;
        let mut out: HashMap<(u32, i32), Vec<String>> = HashMap::new();
        for row in rows.unwrap_or_default() {
            let (Some(rel), Some(att), Some(name), Some(text)) = (row.first().and_then(as_u32), row.get(1).and_then(as_u32), row.get(2).and_then(as_str), row.get(3).and_then(as_str)) else { continue };
            let Some(values) = check_values(text, name) else { continue };
            // Every CHECK holds, so a value is in each one's list.
            out.entry((rel, att as i32)).and_modify(|known| known.retain(|v| values.contains(v))).or_insert(values);
        }
        Some(out)
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
        let nullable_tables: Vec<String> = parsed.map(|p| unreduced(p, trust)).unwrap_or_default();
        let nullable_tables: &[String] = &nullable_tables;
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
            Some(p) => (p.has_repeated_table && !nullable_tables.is_empty()) || p.has_grouping_sets,
            None => true,
        };
        // A set operation anywhere: provenance names the leftmost arm's column, which says
        // nothing about the other arms. The statement's own columns stay nullable; a column read
        // from a WITH query is settled from that query's own probe, whose arms are combined.
        let degraded = hidden || parsed.is_some_and(|p| p.has_set_op);
        // A column read straight from a WITH query, a derived table or a function in FROM is as
        // nullable as that item's own result says, whatever the catalog says of the table column
        // behind it.
        let item_columns = if hidden { Some(Vec::new()) } else { self.item_columns(sql, raw.columns.len(), trust) };
        let Some(item_columns) = item_columns else { return None };
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
                DescribedColumn { name: col.name.clone(), type_oid: col.oid, type_name: registry.name(col.oid), nullable, nullable_items: None, values: None }
            })
            .collect();
        let columns = columns
            .into_iter()
            .enumerate()
            .map(|(i, col)| match item_columns.get(i) {
                // An outer join nulls a WITH query by its name, and any other item by its place.
                Some(Some((with, nullable))) => DescribedColumn { nullable: *nullable || with.as_ref().is_some_and(|query| nullable_tables.iter().any(|t| t == query)), ..col },
                _ => col,
            })
            .collect();
        Some(DescribedStatement { param_oids: raw.params.clone(), columns })
    }

    /// Per result column of `sql` that is a bare reference to a column of one of its FROM items
    /// with a statement of its own ([`infer::ItemQuery`]: a WITH query, a derived table, a
    /// function): the WITH query's name for one, and whether the item's own settled result has
    /// the column nullable, or an outer join nulls an item that is no WITH query. A `*` over such
    /// an item is its columns in order.
    ///
    /// A recursive WITH query reads itself, so its columns' nullability is a fixed point, and
    /// the greatest one is sound. Every column starts as non-null for the reads the query makes
    /// of itself; the query is settled under that assumption; a column that comes out nullable
    /// all the same joins the assumption as nullable, and the query is settled again, until
    /// nothing changes. What survives holds by induction over the iterations: the rows of the
    /// non-recursive arm are non-null on their own account, each later row is non-null given
    /// the rows it was computed from, and a UNION is non-null only where both arms are.
    fn item_columns(&self, sql: &str, count: usize, trust: &Trust) -> Lazy<Vec<Option<(Option<String>, bool)>>> {
        use infer::ItemRead;
        let Some((reads, queries)) = infer::item_reads(sql) else { return Some(Vec::new()) };
        // One `*` at most: its width is what the other targets leave of the result's columns.
        let stars = reads.iter().filter(|r| matches!(r, ItemRead::Star { .. })).count();
        let Some(width) = (count + stars).checked_sub(reads.len()).filter(|w| stars == 1 || (stars == 0 && *w == 0)) else { return Some(Vec::new()) };
        let read = |i: usize| reads.iter().any(|r| matches!(r, ItemRead::Column { item, .. } | ItemRead::Star { item: Some(item) } if *item == i));
        let mut settled: HashMap<usize, Vec<(String, bool)>> = HashMap::new();
        let mut pending = false;
        for (i, query) in queries.iter().enumerate().filter(|(i, _)| read(*i)) {
            // The query reading itself: the answer is the assumption it is being settled under,
            // for the columns it reads by name. A `*` there finds no column list of its width.
            if let Some(entry) = self.with_stack.borrow_mut().iter_mut().find(|(text, _, _)| *text == query.sql) {
                entry.2 = true;
                let assumed = &entry.1;
                let named = reads.iter().filter_map(|r| match r {
                    ItemRead::Column { item, column } if *item == i => Some((column.clone(), assumed.contains(column))),
                    _ => None,
                });
                settled.insert(i, named.collect());
                continue;
            }
            let mut assumed: HashSet<String> = HashSet::new();
            loop {
                self.with_stack.borrow_mut().push((query.sql.clone(), assumed.clone(), false));
                let outcome = self.probe(&query.sql, trust);
                let (_, _, read_itself) = self.with_stack.borrow_mut().pop().expect("the entry pushed above");
                let described = match outcome {
                    Some(Some(described)) => described,
                    // An item the server will not describe alone proves nothing.
                    Some(None) => break,
                    None => {
                        pending = true;
                        break;
                    }
                };
                let columns: Vec<(String, bool)> = described.columns.iter().enumerate().map(|(i, c)| (query.columns.get(i).cloned().unwrap_or_else(|| c.name.clone()), c.nullable)).collect();
                let refuted: Vec<String> = columns.iter().filter(|(n, nullable)| *nullable && !assumed.contains(n)).map(|(n, _)| n.clone()).collect();
                if !read_itself || refuted.is_empty() {
                    settled.insert(i, columns);
                    break;
                }
                assumed.extend(refuted);
            }
        }
        if pending {
            return None;
        }
        let answer = |item: usize, nullable: bool| (queries[item].with.clone(), nullable || (queries[item].with.is_none() && queries[item].nulled));
        let mut out = Vec::with_capacity(count);
        for read in reads {
            match read {
                ItemRead::Other => out.push(None),
                ItemRead::Column { item, column } => {
                    let mut named = settled.get(&item).into_iter().flatten().filter(|(n, _)| *n == column);
                    out.push(match (named.next(), named.next()) {
                        (Some((_, nullable)), None) => Some(answer(item, *nullable)),
                        _ => None,
                    });
                }
                ItemRead::Star { item } => match item.and_then(|item| Some((item, settled.get(&item).filter(|columns| columns.len() == width)?))) {
                    Some((item, columns)) => out.extend(columns.iter().map(|(_, nullable)| Some(answer(item, *nullable)))),
                    None => out.extend(std::iter::repeat(None).take(width)),
                },
            }
        }
        Some(out)
    }

    /// Settle what the describe left open about expression columns from the SQL itself. A column
    /// only ever becomes non-null here, and a probe that cannot be described proves nothing.
    fn refine(&self, sql: &str, described: &mut DescribedStatement, trust: &Trust) -> Lazy<()> {
        let columns = level_columns(sql, self.host)?;
        let plan = infer::plan(sql, Dialect::Postgres, trust, &columns);
        // Per column: whether it is non-null, and what the statement says of an array's elements.
        let facts: Vec<Option<(bool, Option<bool>)>> = match &plan.set_op {
            Some(set) => {
                let left = self.probe(&set.left, trust);
                let right = self.probe(&set.right, trust);
                let (Some(left), Some(right)) = (left, right) else { return None };
                let (Some(left), Some(right)) = (left, right) else { return Some(()) };
                let Some(facts) = infer::combine(set.kind, &left.columns, &right.columns) else { return Some(()) };
                // A set operation's array is one of an arm's, and an INTERSECT's is one of each.
                let items = left.columns.iter().zip(&right.columns).map(|(l, r)| match set.kind {
                    infer::SetKind::Union => match (l.nullable_items, r.nullable_items) {
                        (Some(true), _) | (_, Some(true)) => Some(true),
                        (Some(false), Some(false)) => Some(false),
                        _ => None,
                    },
                    infer::SetKind::Intersect => match (l.nullable_items, r.nullable_items) {
                        (Some(false), _) | (_, Some(false)) => Some(false),
                        (Some(true), _) | (_, Some(true)) => Some(true),
                        _ => None,
                    },
                    infer::SetKind::Except => l.nullable_items,
                });
                facts.into_iter().zip(items).map(|(f, items)| Some((f.non_null, items))).collect()
            }
            None => {
                let claims = plan.claims(described.columns.len());
                let registry = self.registry.borrow();
                let is_array = |o: u32| registry.get(o).is_some_and(|t| t.kind == crate::pgtypes::TypeKind::Array);
                let open = claims.iter().zip(&described.columns).any(|(claim, col)| claim.is_some() && (col.nullable || is_array(col.type_oid)));
                drop(registry);
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
                // A type the registry has not read may be an array, whose elements say nothing.
                let registry = self.registry.borrow();
                let is_array = |o: u32| registry.get(o).is_none_or(|t| t.kind == crate::pgtypes::TypeKind::Array);
                claims.iter().map(|claim| claim.map(|e| (infer::resolve(e, leaves.as_deref()).non_null, infer::items_nullable(e, leaves.as_deref(), &is_array)))).collect()
            }
        };
        if facts.len() != described.columns.len() {
            return Some(());
        }
        for (col, fact) in described.columns.iter_mut().zip(facts) {
            if let Some((non_null, items)) = fact {
                if non_null {
                    col.nullable = false;
                }
                col.nullable_items = items;
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
        self.without_missing_rows(sql, parsed.as_ref(), &raw, &mut described);
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

/// An `ON CONFLICT (a, b)` that names its arbiter index by plain columns alone.
#[derive(Debug, PartialEq)]
struct ConflictTarget {
    /// The INSERT's target, as the statement writes it.
    relation: String,
    columns: Vec<String>,
    /// Whether the columns are followed by a WHERE, which a partial index's predicate is matched by.
    filtered: bool,
    /// Where the column list opens, in the statement's text.
    start: usize,
}

/// The conflict targets of every INSERT in a statement that are lists of plain columns. `ON
/// CONSTRAINT name` names its index, and a target with an expression (`(lower(email))`) is matched
/// against index expressions the build does not compare: neither is one.
fn conflict_targets(sql: &str) -> Vec<ConflictTarget> {
    let Ok(parsed) = crate::stack::parse(sql) else { return Vec::new() };
    let Ok(tree) = serde_json::to_value(&parsed.protobuf) else { return Vec::new() };
    fn walk(v: &Value, out: &mut Vec<ConflictTarget>) {
        match v {
            Value::Object(map) => {
                if let Some(target) = map.get("InsertStmt").and_then(target) {
                    out.push(target);
                }
                map.values().for_each(|child| walk(child, out));
            }
            Value::Array(items) => items.iter().for_each(|child| walk(child, out)),
            _ => {}
        }
    }
    fn target(insert: &Value) -> Option<ConflictTarget> {
        let relation = insert.get("relation")?;
        let name = relation.get("relname")?.as_str()?;
        let relation = match relation.get("schemaname").and_then(Value::as_str) {
            Some(schema) if !schema.is_empty() => format!("{schema}.{name}"),
            _ => name.to_string(),
        };
        let infer = insert.get("on_conflict_clause")?.get("infer")?;
        if infer.get("conname").and_then(Value::as_str).is_some_and(|c| !c.is_empty()) {
            return None;
        }
        let columns: Vec<String> = infer
            .get("index_elems")?
            .as_array()?
            .iter()
            .map(|e| {
                let elem = e.get("node")?.get("IndexElem")?;
                let name = elem.get("name")?.as_str().filter(|n| !n.is_empty())?;
                elem.get("expr").is_none_or(Value::is_null).then(|| name.to_string())
            })
            .collect::<Option<_>>()?;
        if columns.is_empty() {
            return None;
        }
        let filtered = infer.get("where_clause").is_some_and(|w| !w.is_null());
        let start = usize::try_from(infer.get("location")?.as_i64()?).ok()?;
        Some(ConflictTarget { relation, columns, filtered, start })
    }
    let mut out = Vec::new();
    walk(&tree, &mut out);
    out
}

/// Why the server refuses every execution of an INSERT with this conflict target, when it does:
/// no unique index of the table has exactly the target's columns as its key, so the planner infers
/// no arbiter ("there is no unique or exclusion constraint matching the ON CONFLICT
/// specification", raised when the statement is planned, after it prepared). `rows` are
/// [`ARBITERS_QUERY`]'s. An index matches only when its key columns are the target's as a set, it
/// has no expression among them, and its predicate, if it has one, follows from the target's
/// WHERE: an implication the build does not judge, so a partial index of those columns is taken
/// to match a target with a WHERE, and never one without. Nothing is said of a relation the rows
/// do not list, or one a rule rewrites the INSERT of.
fn unmatched_conflict(target: &ConflictTarget, rows: &[Vec<Value>], catalog: &Catalog) -> Option<String> {
    if catalog.rewritten.get(&target.relation).is_some_and(|events| events & cardinality::TRIGGER_ROW_INSERT != 0) {
        return None;
    }
    let mut listed = false;
    let mut partial: Option<&str> = None;
    let wanted: HashSet<&str> = target.columns.iter().map(String::as_str).collect();
    for row in rows.iter().filter(|row| row.first().and_then(as_str) == Some(target.relation.as_str())) {
        listed = true;
        let Some(parts) = row.get(1).and_then(Value::as_array) else { continue };
        let key: Option<HashSet<&str>> = parts.iter().map(Value::as_str).collect();
        if key.is_none_or(|key| key != wanted) {
            continue;
        }
        match row.get(2).and_then(as_str) {
            Some(predicate) if !target.filtered => partial = Some(predicate),
            _ => return None,
        }
    }
    if !listed {
        return None;
    }
    let columns = target.columns.join(", ");
    Some(match partial {
        Some(predicate) => format!(
            "ON CONFLICT ({columns}) matches no unique index of `{}`: its unique index on ({columns}) is partial (WHERE {predicate}), and only a target that repeats the predicate, ON CONFLICT ({columns}) WHERE …, infers it. The server prepares this statement and refuses every execution (42P10).",
            target.relation
        ),
        None => format!(
            "ON CONFLICT ({columns}) matches no unique index or constraint of `{}`: none has exactly ({columns}) as its key. The server prepares this statement and refuses every execution (42P10).",
            target.relation
        ),
    })
}

/// Per result column of a data-modifying statement, whether the provenance the describe reports
/// for it is of a row the statement may not have ([`crate::returning`]): the old row of an INSERT,
/// the new row of a DELETE, either by the name RETURNING gives it, or a MERGE's source where the
/// MERGE acts on a target row the source does not match. Such a column is NULL there whatever its
/// table's `NOT NULL` says. `origins` says which result columns the describe traced to a table's
/// column (a reference, also under a cast that changes nothing, and a field of a whole row,
/// `(old).name`); an expression that carries no provenance is the expression rules' to settle,
/// which read the same rows. `columns` are a relation's column names, in order: what a `*` expands
/// to, and what a bare name is before it is a row. When the statement's columns cannot be told
/// apart (a `*` over more than the target relation, or over a relation of unknown columns, or a
/// data-modifying WITH query that reads a missing row, whose columns the statement can read
/// through any expression), every column may be NULL.
fn missing_rows(sql: &str, origins: &[bool], columns: &dyn Fn(&str) -> Option<Vec<String>>) -> Vec<bool> {
    let count = origins.len();
    let none = vec![false; count];
    let Ok(parsed) = crate::stack::parse(sql) else { return none };
    let Some(top) = parsed.protobuf.stmts.first().and_then(|s| s.stmt.as_deref()).and_then(|n| n.node.as_ref()) else { return none };
    // (the rows RETURNING can name, the target relation, the RETURNING list)
    let shape = |node: &NodeEnum| -> Option<(Rows, String, Vec<pg_query::protobuf::Node>)> {
        let (relation, clause) = match node {
            NodeEnum::InsertStmt(s) => (s.relation.as_ref()?, &s.returning_clause),
            NodeEnum::UpdateStmt(s) => (s.relation.as_ref()?, &s.returning_clause),
            NodeEnum::DeleteStmt(s) => (s.relation.as_ref()?, &s.returning_clause),
            NodeEnum::MergeStmt(s) => (s.relation.as_ref()?, &s.returning_clause),
            _ => return None,
        };
        Some((Rows::of(node)?, relation.relname.clone(), jsonshape::returning(clause)))
    };
    let reads_missing = |rows: &Rows, relation: &str, node: &pg_query::protobuf::Node| {
        let known = columns(relation);
        rows.reads_missing(node, &|name| known.as_ref().is_some_and(|k| k.iter().any(|c| c == name)))
    };
    // A data-modifying WITH query under the statement: its RETURNING can feed any column.
    let with = match top {
        NodeEnum::SelectStmt(s) => s.with_clause.as_ref(),
        NodeEnum::InsertStmt(i) => i.with_clause.as_ref(),
        NodeEnum::UpdateStmt(u) => u.with_clause.as_ref(),
        NodeEnum::DeleteStmt(d) => d.with_clause.as_ref(),
        NodeEnum::MergeStmt(m) => m.with_clause.as_ref(),
        _ => None,
    };
    for cte in with.map(|w| w.ctes.as_slice()).unwrap_or_default() {
        let Some(NodeEnum::CommonTableExpr(c)) = cte.node.as_ref() else { continue };
        let Some((rows, relation, returning)) = c.ctequery.as_deref().and_then(|q| q.node.as_ref()).and_then(&shape) else { continue };
        if !rows.all_present() && returning.iter().any(|r| reads_missing(&rows, &relation, r)) {
            return vec![true; count];
        }
    }
    let Some((rows, relation, returning)) = shape(top) else { return none };
    if rows.all_present() {
        return none;
    }
    let mut out = Vec::with_capacity(count);
    for target in &returning {
        let Some(NodeEnum::ResTarget(rt)) = target.node.as_ref() else { return vec![true; count] };
        let Some(val) = rt.val.as_deref() else { return vec![true; count] };
        let star = match val.node.as_ref() {
            Some(NodeEnum::ColumnRef(c)) => match c.fields.split_last() {
                Some((last, qualifier)) if matches!(last.node, Some(NodeEnum::AStar(_))) => Some(qualifier),
                _ => None,
            },
            _ => None,
        };
        let missing = reads_missing(&rows, &relation, val);
        match star {
            None => out.push(missing),
            // `o.*`, the target's own `*`, and a `*` with nothing joined in are the target
            // relation's columns; any other has a width the parse does not show.
            Some(qualifier) => {
                let qualifier = match qualifier {
                    [] => Some(None),
                    [one] => jsonshape::last_string(std::slice::from_ref(one)).map(Some),
                    _ => None,
                };
                let known = qualifier.filter(|q| rows.star_of_target(q.as_deref())).and_then(|_| columns(&relation));
                let Some(known) = known else { return vec![true; count] };
                out.extend(std::iter::repeat(missing).take(known.len()));
            }
        }
    }
    if out.len() != count {
        return vec![true; count];
    }
    out.iter().zip(origins).map(|(missing, origin)| *missing && *origin).collect()
}

/// `ty` as the union of `values` when it is a string a CHECK keeps to them; the labels of an enum
/// say the same of a column whose type is one.
pub fn with_values(ty: Type, values: Option<&[String]>) -> Type {
    match (ty, values) {
        (Type::Scalar { name }, Some(values)) if (name == "text" || name == "varchar") && !values.is_empty() => Type::Enum { name, labels: values.to_vec() },
        (ty, _) => ty,
    }
}

/// The strings a CHECK, as the server prints it, keeps the column `column` to (`VALUE` in a
/// domain's): `col = ANY (ARRAY['a'::text, …])` or `col = 'a'`, alone, among the conjuncts of an
/// AND (each one of them restricts), or beside `col IS NULL` under an OR, which only lets NULL
/// through as well. A cast to a string type around the column or a value is the server's own
/// (`v::text = ANY (ARRAY['x'::character varying]::text[])` for a `varchar`). Anything else
/// proves nothing.
fn check_values(text: &str, column: &str) -> Option<Vec<String>> {
    use pg_query::protobuf::node::Node as N;
    let parsed = crate::stack::parse(&format!("SELECT {text}")).ok()?;
    let Some(N::SelectStmt(select)) = parsed.protobuf.stmts.first()?.stmt.as_deref()?.node.as_ref() else { return None };
    let Some(N::ResTarget(target)) = select.target_list.first()?.node.as_ref() else { return None };
    fn uncast(node: &pg_query::protobuf::Node) -> &pg_query::protobuf::Node {
        match node.node.as_ref() {
            Some(N::TypeCast(t)) => {
                let name = t.type_name.as_ref().and_then(|n| crate::jsonshape::last_string(&n.names)).unwrap_or_default();
                match (["text", "varchar"].contains(&name.as_str()), t.arg.as_deref()) {
                    (true, Some(arg)) => uncast(arg),
                    _ => node,
                }
            }
            _ => node,
        }
    }
    let is_column = |node: &pg_query::protobuf::Node| match uncast(node).node.as_ref() {
        Some(N::ColumnRef(c)) => matches!(c.fields.as_slice(), [f] if matches!(f.node.as_ref(), Some(N::String(s)) if s.sval == column || s.sval == "value")),
        _ => false,
    };
    let string = |node: &pg_query::protobuf::Node| match uncast(node).node.as_ref() {
        Some(N::AConst(c)) => match &c.val {
            Some(pg_query::protobuf::a_const::Val::Sval(s)) => Some(s.sval.clone()),
            _ => None,
        },
        _ => None,
    };
    fn list(node: &pg_query::protobuf::Node, is_column: &dyn Fn(&pg_query::protobuf::Node) -> bool, string: &dyn Fn(&pg_query::protobuf::Node) -> Option<String>) -> Option<Vec<String>> {
        use pg_query::protobuf::{AExprKind, BoolExprType, NullTestType};
        match node.node.as_ref()? {
            N::BoolExpr(b) if b.boolop == BoolExprType::AndExpr as i32 => {
                let mut lists = b.args.iter().filter_map(|a| list(a, is_column, string));
                let first = lists.next()?;
                Some(lists.fold(first, |kept, other| kept.into_iter().filter(|v| other.contains(v)).collect()))
            }
            N::BoolExpr(b) if b.boolop == BoolExprType::OrExpr as i32 && b.args.len() == 2 => {
                let null_test = |a: &pg_query::protobuf::Node| matches!(a.node.as_ref(), Some(N::NullTest(t)) if t.nulltesttype == NullTestType::IsNull as i32 && t.arg.as_deref().is_some_and(is_column));
                match (null_test(&b.args[0]), null_test(&b.args[1])) {
                    (true, false) => list(&b.args[1], is_column, string),
                    (false, true) => list(&b.args[0], is_column, string),
                    _ => None,
                }
            }
            N::AExpr(e) if crate::jsonshape::last_string(&e.name).as_deref() == Some("=") && e.name.len() == 1 => {
                let (l, r) = (e.lexpr.as_deref()?, e.rexpr.as_deref()?);
                if e.kind == AExprKind::AexprOpAny as i32 && is_column(l) {
                    let array = match r.node.as_ref()? {
                        N::TypeCast(t) => t.arg.as_deref()?,
                        _ => r,
                    };
                    let Some(N::AArrayExpr(a)) = array.node.as_ref() else { return None };
                    a.elements.iter().map(string).collect()
                } else if e.kind == AExprKind::AexprOp as i32 {
                    match (is_column(l), is_column(r)) {
                        (true, false) => string(r).map(|v| vec![v]),
                        (false, true) => string(l).map(|v| vec![v]),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    list(target.val.as_deref()?, &is_column, &string)
}

/// `ty` with what the statement says of its elements, when it is an array.
pub fn with_items(ty: Type, nullable_items: Option<bool>) -> Type {
    match ty {
        Type::Array { of, .. } => Type::Array { of, nullable_items },
        other => other,
    }
}

/// The tables on the nullable side of an outer join, with the ones a strict WHERE reduced only by
/// operators the catalog says a user or an extension also defined put back: such an operator may
/// be true of NULL, so its conjunct does not prove the joined row exists.
fn unreduced(parsed: &Parsed, trust: &Trust) -> Vec<String> {
    let mut tables = parsed.nullable_tables.clone();
    for (relation, witnesses) in &parsed.reductions {
        if witnesses.iter().all(|names| names.iter().any(|n| !trust.holds(n))) {
            tables.push(relation.clone());
        }
    }
    tables
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

fn build_catalog(operators: &HashSet<String>, rows: CatalogRows) -> Catalog {
    let mut catalog = Catalog { set_returning_operators: operators.clone(), columns: Columns::from_rows(&rows.columns), ..Catalog::default() };
    for row in rows.columns {
        if let (Some(rel), Some(cols)) = (row.first().and_then(as_str), row.get(1).and_then(as_strings)) {
            let types: Vec<Option<u32>> = row.get(2).and_then(Value::as_array).map(|a| a.iter().map(as_u32).collect()).unwrap_or_default();
            // A host that answers without the fourth column reports no collation: every column
            // is then taken as one whose equality is not identity.
            let deterministic: Vec<bool> = row.get(3).and_then(Value::as_array).map(|a| a.iter().map(|v| as_bool(v).unwrap_or(false)).collect()).unwrap_or_default();
            // A host that answers without the fifth column reports no sequence.
            if row.get(4).and_then(as_bool).unwrap_or(false) {
                catalog.single_row.insert(rel.to_string());
            }
            if types.len() == cols.len() {
                for (i, col) in cols.iter().enumerate() {
                    if !deterministic.get(i).copied().unwrap_or(false) {
                        catalog.nondeterministic.entry(rel.to_string()).or_default().insert(col.clone());
                    }
                }
                let known = catalog.column_types.entry(rel.to_string()).or_default();
                for (col, ty) in cols.into_iter().zip(types) {
                    if let Some(ty) = ty {
                        known.insert(col, ty);
                    }
                }
            }
        }
    }
    for row in rows.keys {
        if let (Some(rel), Some(parts)) = (row.first().and_then(as_str), row.get(1).and_then(as_strings)) {
            // An expression part's type goes under the part's own name (`(lower(email))`), beside
            // the relation's columns, for the comparison with it.
            let types: Vec<Option<u32>> = row.get(2).and_then(Value::as_array).map(|a| a.iter().map(as_u32).collect()).unwrap_or_default();
            if let Some(known) = catalog.column_types.get_mut(rel) {
                for (part, ty) in parts.iter().zip(types) {
                    if let (true, Some(ty)) = (part.starts_with('('), ty) {
                        known.insert(part.clone(), ty);
                    }
                }
            }
            match row.get(3).and_then(as_str) {
                Some(predicate) => catalog.partial_keys.entry(rel.to_string()).or_default().push((parts, predicate.to_string())),
                None => catalog.unique_keys.entry(rel.to_string()).or_default().push(parts),
            }
        }
    }
    for row in rows.triggers {
        if let (Some(rel), Some(events)) = (row.first().and_then(as_str), row.get(1).and_then(as_u32)) {
            if events != 0 {
                catalog.skipping_triggers.insert(rel.to_string(), events as i32);
            }
            // A host that answers without the third column reports no rules: every write is then
            // taken as one a rule may rewrite.
            let rewritten = row.get(2).map_or(Some(28), as_u32).unwrap_or(0);
            if rewritten != 0 {
                catalog.rewritten.insert(rel.to_string(), rewritten as i32);
            }
        }
    }
    for row in rows.functions {
        if let (Some(name), Some(retset)) = (row.first().and_then(as_str), row.get(1).and_then(as_bool)) {
            if retset {
                catalog.set_returning.insert(name.to_string());
            }
            // A host that answers without the third column reports no volatility: every call is
            // then taken as volatile, which refuses a key pinned to any call.
            if row.get(2).and_then(as_bool).unwrap_or(true) {
                catalog.volatile.insert(name.to_string());
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
        let parsed = crate::stack::parse(sql).expect("the probe parses");
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
    fn each_parameter_appears_once() {
        // Every later use is an error at its place, before the server is asked anything, and the
        // parameters stay listed, each once, untyped.
        let sql = "select id, name from users where name like :q or email like :q or nick like :q";
        let none = HashMap::new();
        let host = Host::new(&none);
        let a = analyze(sql, Dialect::Postgres, &host).expect("refused without a round trip");
        assert!(host.needs().is_empty());
        let second = sql.find(":q or nick").unwrap();
        assert_eq!(a.diagnostics.iter().map(|d| (d.code, d.start, d.length)).collect::<Vec<_>>(), [(3, second, 2), (3, sql.rfind(":q").unwrap(), 2)]);
        assert!(a.diagnostics[0].message.starts_with("The parameter :q appears more than once; use each parameter once"));
        assert_eq!(a.params.iter().map(|p| (p.name.as_str(), &p.ty)).collect::<Vec<_>>(), [("q", &Type::Unknown)]);
        let a = analyze("select $1::int + $1", Dialect::Postgres, &host).unwrap();
        assert_eq!(a.diagnostics.iter().map(|d| (d.code, d.start)).collect::<Vec<_>>(), [(3, 17)]);
        // The fix reads the value from a WITH.
        let a = replay("with input as (select concat('%', :q::text, '%') as pattern) select 1 as one from input where input.pattern like input.pattern");
        assert!(a.diagnostics.iter().all(|d| d.code != 3), "{:?}", a.diagnostics);
    }

    #[test]
    fn a_check_keeps_a_string_column_to_the_list_it_prints() {
        let list = |items: &[&str]| Some(items.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        assert_eq!(check_values("status = ANY (ARRAY['active'::text, 'banned'::text])", "status"), list(&["active", "banned"]));
        assert_eq!(check_values("v::text = ANY (ARRAY['x'::character varying, 'y'::character varying]::text[])", "v"), list(&["x", "y"]));
        assert_eq!(check_values("VALUE = ANY (ARRAY['ok'::text, 'sad'::text])", "mood"), list(&["ok", "sad"]));
        assert_eq!(check_values("status IS NULL OR status = ANY (ARRAY['a'::text, 'b'::text])", "status"), list(&["a", "b"]));
        assert_eq!(check_values("status = ANY (ARRAY['a'::text, 'b'::text]) AND length(status) > 0", "status"), list(&["a", "b"]));
        assert_eq!(check_values("status = 'active'::text", "status"), list(&["active"]));
        // A NULL in the list lets any value pass (`'z' = NULL` is NULL, and a CHECK passes on NULL).
        assert_eq!(check_values("status = ANY (ARRAY['a'::text, NULL::text])", "status"), None);
        for other in [
            "status <> ALL (ARRAY['a'::text, 'b'::text])",
            "status = ANY (ARRAY['a'::text]) OR status = 'b'::text",
            "lower(status) = ANY (ARRAY['a'::text])",
            "other = ANY (ARRAY['a'::text])",
            "status OPERATOR(public.=) 'a'::text",
        ] {
            assert_eq!(check_values(other, "status"), None, "{other}");
        }
        assert_eq!(with_values(Type::Scalar { name: "text".into() }, Some(&["a".to_string()])), Type::Enum { name: "text".into(), labels: vec!["a".into()] });
        assert_eq!(with_values(Type::Scalar { name: "bpchar".into() }, Some(&["a".to_string()])), Type::Scalar { name: "bpchar".into() });
    }

    #[test]
    fn a_conflict_target_no_unique_index_matches_is_refused() {
        let target = |sql: &str| conflict_targets(sql).into_iter().next();
        let name = target("INSERT INTO bar (id, name) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING").expect("a target");
        assert_eq!(name, ConflictTarget { relation: "bar".into(), columns: vec!["name".into()], filtered: false, start: 55 });
        let pair = target("INSERT INTO app.bar AS b (id, name) VALUES ($1, $2) ON CONFLICT (name, id) WHERE id > 0 DO UPDATE SET name = excluded.name").expect("a target");
        assert_eq!((pair.relation.as_str(), pair.columns.len(), pair.filtered), ("app.bar", 2, true));
        // One inside a WITH query is read too.
        assert_eq!(conflict_targets("WITH i AS (INSERT INTO bar (id) VALUES ($1) ON CONFLICT (id) DO NOTHING RETURNING id) SELECT id FROM i").len(), 1);
        // No target, a named constraint and an expression are not lists of columns.
        for sql in [
            "INSERT INTO bar (id) VALUES ($1)",
            "INSERT INTO bar (id) VALUES ($1) ON CONFLICT DO NOTHING",
            "INSERT INTO bar (id) VALUES ($1) ON CONFLICT ON CONSTRAINT bar_pkey DO NOTHING",
            "INSERT INTO bar (id, name) VALUES ($1, $2) ON CONFLICT (lower(name)) DO NOTHING",
            "INSERT INTO bar (id, name) VALUES ($1, $2) ON CONFLICT (id, (name || 'x')) DO NOTHING",
        ] {
            assert_eq!(target(sql), None, "{sql}");
        }

        // `bar` has a primary key on id, a unique index on (a, b), one on lower(name), and a
        // partial one on email; `empty` has no unique index; `v` is a view and has no row.
        let rows = vec![
            vec![json!("bar"), json!(["id"]), Value::Null],
            vec![json!("bar"), json!(["b", "a"]), Value::Null],
            vec![json!("bar"), json!([null]), Value::Null],
            vec![json!("bar"), json!(["email"]), json!("active")],
            vec![json!("empty"), Value::Null, Value::Null],
        ];
        let check = |relation: &str, columns: &[&str], filtered: bool, catalog: &Catalog| {
            unmatched_conflict(&ConflictTarget { relation: relation.into(), columns: columns.iter().map(|c| c.to_string()).collect(), filtered, start: 0 }, &rows, catalog)
        };
        let none = Catalog::default();
        assert_eq!(check("bar", &["id"], false, &none), None);
        assert_eq!(check("bar", &["a", "b"], false, &none), None, "the key's columns in any order");
        assert_eq!(check("bar", &["id", "id"], false, &none), None, "a column named twice is one");
        assert!(check("bar", &["name"], false, &none).is_some_and(|m| m.contains("none has exactly (name)")));
        assert!(check("bar", &["a"], false, &none).is_some(), "half of a key");
        assert!(check("bar", &["a", "b", "id"], false, &none).is_some(), "more than a key");
        assert!(check("empty", &["id"], false, &none).is_some());
        // A partial index is inferred only by a target with a WHERE that implies its predicate.
        assert!(check("bar", &["email"], false, &none).is_some_and(|m| m.contains("is partial (WHERE active)")));
        assert_eq!(check("bar", &["email"], true, &none), None);
        // A relation the rows do not list, and one whose INSERT a rule rewrites.
        assert_eq!(check("v", &["id"], false, &none), None);
        let mut ruled = Catalog::default();
        ruled.rewritten.insert("empty".into(), cardinality::TRIGGER_ROW_INSERT);
        assert_eq!(check("empty", &["id"], false, &ruled), None);
    }

    #[test]
    fn returning_reads_a_row_the_statement_may_not_have() {
        let columns = |relation: &str| (relation != "unknown").then(|| vec!["id".to_string(), "name".to_string()]);
        // Every column traced to a table's column, which is what the answer is about.
        let missing = |sql: &str, count: usize| missing_rows(sql, &vec![true; count], &columns);
        assert_eq!(missing("INSERT INTO users (id, name) VALUES ($1, $2) RETURNING old.name, new.name, name", 3), vec![true, false, false]);
        assert_eq!(missing("DELETE FROM users WHERE id = $1 RETURNING old.name, new.name, new.*", 4), vec![false, true, true, true]);
        assert_eq!(missing("UPDATE users SET name = $1 WHERE id = $2 RETURNING old.name, new.name", 2), vec![false, false]);
        assert_eq!(missing("INSERT INTO users VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET name = excluded.name RETURNING old.*", 2), vec![true, true]);
        // By the names the clause gives the rows, which hide the default ones.
        assert_eq!(missing("INSERT INTO users (id, name) VALUES ($1, $2) RETURNING WITH (OLD AS o, NEW AS n) o.name, n.name, o.*, n.*", 6), vec![true, false, true, true, false, false]);
        assert_eq!(missing("DELETE FROM users WHERE id = $1 RETURNING WITH (NEW AS n) n.name, old.name, *", 4), vec![true, false, false, false]);
        // A relation of the name is read first: the table `old`, the target under the alias `new`.
        assert_eq!(missing("INSERT INTO old (id, name) VALUES ($1, $2) RETURNING old.name, new.name", 2), vec![false, false]);
        assert_eq!(missing("DELETE FROM users AS new WHERE id = $1 RETURNING new.name, old.name", 2), vec![false, false]);
        assert_eq!(missing("INSERT INTO old (id, name) VALUES ($1, $2) RETURNING WITH (OLD AS was) was.name, old.name", 2), vec![true, false]);
        // A field of the whole row and a cast that changes nothing keep their provenance.
        assert_eq!(missing("INSERT INTO users (id, name) VALUES ($1, $2) RETURNING (old).name, old.name::text, (new).name", 3), vec![true, true, false]);
        // A bare name is a column before it is a row.
        assert_eq!(missing("INSERT INTO users (id, name) VALUES ($1, $2) RETURNING WITH (OLD AS name) name, name.id", 2), vec![false, true]);
        // An expression carries no provenance: the expression rules settle it.
        assert_eq!(missing_rows("INSERT INTO users (id, name) VALUES ($1, $2) RETURNING old.name, old.name || '!', old.id IS NULL", &[true, false, false], &columns), vec![true, false, false]);
        // A MERGE that inserts has no old row for that action; one that never deletes always has a new one.
        let merge = "MERGE INTO users u USING (SELECT 1 AS id) s ON u.id = s.id WHEN MATCHED THEN UPDATE SET name = 'x' WHEN NOT MATCHED THEN INSERT (id, name) VALUES (s.id, 'n') RETURNING old.name, new.name, s.id";
        assert_eq!(missing(merge, 3), vec![true, false, false]);
        // One that acts where the source matches nothing returns rows with no source row.
        let by_source = "MERGE INTO users u USING src s ON u.id = s.id WHEN MATCHED THEN UPDATE SET name = s.name WHEN NOT MATCHED BY SOURCE THEN UPDATE SET name = 'gone' RETURNING s.name, u.name, name, label, old.name";
        assert_eq!(missing(by_source, 5), vec![true, false, false, true, false]);
        // A data-modifying WITH query that reads a missing row may feed any column.
        assert_eq!(missing("WITH i AS (INSERT INTO users VALUES ($1, 'n') RETURNING old.name AS n) SELECT n, 1 FROM i", 2), vec![true, true]);
        assert_eq!(missing("WITH i AS (INSERT INTO users VALUES ($1, 'n') RETURNING WITH (OLD AS o) o.name AS n) SELECT n, 1 FROM i", 2), vec![true, true]);
        assert_eq!(missing("WITH i AS (INSERT INTO users VALUES ($1, 'n') RETURNING new.name AS n) SELECT n, 1 FROM i", 2), vec![false, false]);
        // A `*` over a relation of unknown columns, or over more than the target, hides which column is which.
        assert_eq!(missing("INSERT INTO unknown VALUES ($1, $2) RETURNING new.*, old.name", 3), vec![true, true, true]);
        assert_eq!(missing("DELETE FROM users USING other WHERE other.id = users.id RETURNING *, new.name", 5), vec![true, true, true, true, true]);
        assert_eq!(missing("SELECT 1, 2", 2), vec![false, false]);
    }

    /// A server with one table, `users (id int primary key, name text not null)`, that describes
    /// every statement as `name` columns of it, one per name in `names`, and types `$1` and `$2`
    /// as text and int.
    fn replay_users(sql: &str, names: &[&str]) -> Analysis {
        let mut answers: HashMap<String, Answer> = HashMap::new();
        for _ in 0..64 {
            let host = Host::new(&answers);
            if let Some(analysis) = analyze(sql, Dialect::Postgres, &host) {
                return analysis;
            }
            for need in host.needs() {
                match need {
                    crate::api::Need::Describe { id, .. } => {
                        let columns = names.iter().map(|name| RawColumn { name: name.to_string(), oid: 25, table: 16400, attnum: 2 }).collect();
                        answers.insert(id, Answer::Describe { describe: RawDescribe { params: vec![25, 23], columns } });
                    }
                    crate::api::Need::Query { id, sql, .. } => {
                        let rows = match sql.as_str() {
                            RELATION_COLUMNS_QUERY => vec![
                                vec![json!(16400), json!("users"), json!(1), json!(true), Value::Null, json!("id"), json!(true)],
                                vec![json!(16400), json!("users"), json!(2), json!(true), Value::Null, json!("name"), json!(true)],
                            ],
                            cardinality::UNIQUE_KEYS_QUERY => vec![vec![json!("users"), json!(["id"]), json!([null]), Value::Null]],
                            cardinality::COLUMNS_QUERY => vec![vec![json!("users"), json!(["id", "name"]), json!([23, 25]), json!([true, true]), json!(false)]],
                            _ => vec![],
                        };
                        answers.insert(id, Answer::Rows { rows });
                    }
                }
            }
        }
        panic!("the replay did not converge");
    }

    #[test]
    fn returning_with_names_for_the_old_and_the_new_row_is_typed_as_old_and_new_are() {
        let nullable = |a: &Analysis| a.columns.iter().map(|c| c.nullable).collect::<Vec<_>>();
        // PostgreSQL 18's clause parses, so the write by key is at most one row, and both rows of
        // an UPDATE hold the table's NOT NULL.
        let update = replay_users("UPDATE users SET name = :name WHERE id = :id RETURNING WITH (OLD AS o, NEW AS n) o.name AS old_name, n.name AS new_name", &["old_name", "new_name"]);
        assert_eq!((update.kind.as_str(), update.cardinality.as_str(), update.cardinality_reason.as_deref()), ("update", "maybeOne", None));
        assert!(update.diagnostics.is_empty(), "{:?}", update.diagnostics);
        assert_eq!(nullable(&update), [false, false]);
        assert_eq!(update.params.iter().map(|p| (p.name.as_str(), p.type_name.as_str())).collect::<Vec<_>>(), [("name", "text"), ("id", "int4")]);
        assert_eq!(nullable(&replay_users("UPDATE users SET name = :name WHERE id = :id RETURNING old.name AS old_name, new.name AS new_name", &["old_name", "new_name"])), [false, false]);
        // An INSERT has no old row and a DELETE no new one, by the clause's names as by the default ones.
        let insert = replay_users("INSERT INTO users (name, id) VALUES (:name, :id) RETURNING WITH (OLD AS o, NEW AS n) o.name AS old_name, n.name AS new_name", &["old_name", "new_name"]);
        assert_eq!((insert.cardinality.as_str(), nullable(&insert)), ("one", vec![true, false]));
        let delete = replay_users("DELETE FROM users WHERE name = :name AND id = :id RETURNING WITH (NEW AS after) old.name AS old_name, after.name AS new_name, name", &["old_name", "new_name", "name"]);
        assert_eq!((delete.cardinality.as_str(), nullable(&delete)), ("maybeOne", vec![false, true, false]));
    }

    #[test]
    fn a_statement_at_the_depth_bound_is_analyzed_whole_and_one_past_it_falls_back_to_the_describe() {
        // From a thread with a small stack, as a tokio worker or a Postgres backend may call: the
        // parse, the walks of the expression rules, the row-count proof and the JSON shapes all run
        // on the engine's own stack.
        let small = |sql: String| std::thread::scope(|s| std::thread::Builder::new().stack_size(256 << 10).spawn_scoped(s, move || replay_users(&sql, &["x"])).unwrap().join().unwrap());
        for &(shape, n) in crate::stack::BOUNDS {
            // Keyed where the shape leaves the WHERE free, so the proof has something to prove
            // (`replay_users` types `$1` as text and `$2` as int).
            let sql = crate::stack::nested(shape, n);
            let keyed = !sql.contains(" where ");
            let at = small(if keyed { format!("{sql} where name = :name and id = :id") } else { sql });
            // The key proves at most one row, but for `json_build_object`, which this replay's
            // catalog does not hold.
            let rows = if keyed && shape != "json" { "maybeOne" } else { "many" };
            assert_eq!((at.kind.as_str(), at.cardinality.as_str()), ("select", rows), "{shape} at {n}: {:?}", at.cardinality_reason);
            let past = small(crate::stack::nested(shape, n + 1));
            assert_eq!((past.kind.as_str(), past.cardinality.as_str()), ("", "many"), "{shape} at {}", n + 1);
            assert_eq!(past.cardinality_reason.as_deref(), Some("the statement did not parse locally"));
            assert!(past.columns.iter().all(|c| c.nullable), "{shape} at {}", n + 1);
            assert!(past.diagnostics.is_empty(), "{:?}", past.diagnostics);
        }
    }

    #[test]
    fn the_trust_query_says_whether_the_server_casts_a_jsonb_null() {
        let read = |rows: Vec<Vec<Value>>| {
            let mut answers: HashMap<String, Answer> = HashMap::new();
            let host = Host::new(&answers);
            assert!(trust(&host).is_none());
            let needs = host.needs();
            let [crate::api::Need::Query { id, sql, .. }] = needs.as_slice() else { panic!("one query, not {needs:?}") };
            assert_eq!(sql, TRUST_QUERY);
            answers.insert(id.clone(), Answer::Rows { rows });
            trust(&Host::new(&answers)).unwrap()
        };
        // A server before 18 answers with one row more, a NULL beside the name: its cast of a
        // JSON null raises an error, where 18 returns an SQL NULL.
        let before = read(vec![vec![json!(false), json!("lower")], vec![json!(true), json!("int4")], vec![Value::Null, json!("jsonb casts raise")]]);
        assert!(before.jsonb_casts_raise);
        assert!(before.shadowed.contains("lower") && before.cast_targets.as_ref().unwrap().contains("int4"));
        assert!(!before.shadowed.contains("jsonb casts raise") && !before.cast_targets.as_ref().unwrap().contains("jsonb casts raise"));
        let later = read(vec![vec![json!(false), json!("lower")], vec![json!(true), json!("int4")]]);
        assert!(!later.jsonb_casts_raise);
        // A NULL beside any other name says nothing.
        assert!(!read(vec![vec![Value::Null, json!("lower")]]).jsonb_casts_raise);
    }

    #[test]
    fn a_position_the_parse_reports_is_an_offset_in_the_text_it_read() {
        // libpg_query 18 leaves comments out of what it parses; a location is still an offset in
        // the text with them, which is where a diagnostic is put.
        let sql = "/* upsert */ INSERT INTO bar (id, name) -- the row\n VALUES ($1, $2) /* é */ ON CONFLICT (name) DO NOTHING";
        let target = conflict_targets(sql).into_iter().next().expect("a target");
        assert_eq!(&sql[target.start..target.start + 6], "(name)");
        let compare = "SELECT 1 /* é */ FROM bar -- by name\n WHERE name = $1 AND /* id */ id > $2";
        let at = |needle: &str| compare[..compare.find(needle).unwrap()].chars().count() + 1;
        assert_eq!(analyze::params_at_position(compare, at("= $1")), [1]);
        assert_eq!(analyze::params_at_position(compare, at("$2")), [2]);
        assert_eq!(analyze::params_at_position(compare, at("> $2")), [2]);
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
