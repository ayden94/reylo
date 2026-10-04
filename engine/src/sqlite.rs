//! The SQLite analysis, over SQLite's own resolved tree ([`crate::sqlite_tree`], produced by
//! `csrc/porg_sqlite.c` from a patched amalgamation). SQLite resolves the names, the join sides,
//! every parameter's context and the DML target columns itself, and reports each expression's
//! affinity. What SQLite has no facts for stays here, as explicit tables: the result type and
//! nullability of the built-in functions, the type a function gives its arguments, the arithmetic
//! rule, and the row-count proof from the unique keys. The host runs one query for the schema DDL;
//! everything else happens inside the engine.
//!
//! Two entries share the code: [`analyze`], replayed through the [`Host`] like every dialect, and
//! [`analyze_ddl`], for a host that holds a live connection and runs [`DDL_QUERY`] on it itself,
//! with [`Extras`] naming the scalar functions it registers there.

use crate::api::{Analysis, Column, Diagnostic, Param, Scope};
use crate::dialect::Dialect;
use crate::host::{as_str, Host, Lazy};
use crate::lex::{self, Lexed, Target};
use crate::pgtypes::oid;
use crate::sqlite_tree::{self as tree, From, Node, Sel, Tree};
use crate::types::Type;
use std::collections::{HashMap, HashSet};

/// The schema, as SQLite stores it: its `sql` column is the DDL the engine replays into its
/// in-memory copy. The replayed entry asks the host for it; a host with a connection of its own
/// runs it there and hands the rows to [`analyze_ddl`].
pub const DDL_QUERY: &str = "select sql from sqlite_schema where sql is not null and type in ('table', 'view', 'index', 'trigger') order by rowid";

/// A scalar function the host registers on its own connection, which the engine's replay cannot
/// see: the replay registers a stub under the same name and argument count, so a statement calling
/// it prepares, and the function table types a call as `returns`, NULL only when an argument is.
/// The meaning of [`crate::infer::Trust::total`], plus the argument count SQLite resolves a call by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TotalFunction {
    pub name: String,
    /// How many arguments it takes; -1 for any number.
    pub arity: i32,
    /// The type oid it returns ([`crate::pgtypes::oid`]), read as the storage class it lands in.
    pub returns: u32,
}

/// What a host adds to SQLite on its own connection.
#[derive(Debug, Clone, Default)]
pub struct Extras {
    pub total: Vec<TotalFunction>,
}

impl Extras {
    /// The registration a call resolves to: SQLite prefers a host function over a built-in of the
    /// same name and argument count.
    fn function(&self, name: &str, args: usize) -> Option<&TotalFunction> {
        self.total.iter().find(|f| f.name.eq_ignore_ascii_case(name) && (f.arity < 0 || f.arity as usize == args))
    }
}

/// The storage class a value of this oid lands in on SQLite.
fn oid_affinity(type_oid: u32) -> &'static str {
    match type_oid {
        oid::BOOL | oid::INT2 | oid::INT4 | oid::INT8 => "integer",
        oid::FLOAT4 | oid::FLOAT8 => "real",
        // 1042 is bpchar, 17 bytea, 1700 numeric.
        oid::TEXT | oid::VARCHAR | oid::JSON | oid::JSONB | oid::UUID | 1042 => "text",
        17 => "blob",
        1700 => "numeric",
        _ => "none",
    }
}

/// [`analyze_ddl`]'s answer: the analysis, and what the tree adds for a host that types a result
/// column by the spelling its table declares.
#[derive(Debug, Clone)]
pub struct SqliteAnalysis {
    pub analysis: Analysis,
    /// Per result column, the declared type of the table column it reads
    /// (`sqlite3_column_decltype`, through subqueries and views); `None` for an expression. Empty
    /// when the statement did not prepare.
    pub declared: Vec<Option<String>>,
    /// `sqlite3_stmt_readonly`; false when the statement did not prepare.
    pub readonly: bool,
    /// The unique keys of every table the statement reads or writes (through views and
    /// subqueries), by the table's name as its DDL spells it: every unique, non-partial index over
    /// plain columns, the `INTEGER PRIMARY KEY`, and `rowid` for a rowid table without one. What
    /// the row-count proof reads, for a host that runs that proof over text of its own. Empty when
    /// the statement did not prepare.
    pub unique_keys: HashMap<String, Vec<Vec<String>>>,
    /// Per result column, whether it is a truth value: a comparison, `IN`, `BETWEEN`, `EXISTS`,
    /// `IS`, `IS NULL`, `NOT`, `AND`, `OR`, `TRUE` or `FALSE`, a `like`, `glob`, `regexp` or `match`
    /// call, or a `CASE`, `coalesce`, `ifnull`, `iif`, scalar subquery or subquery column whose
    /// every value is one. SQLite stores one as the integer 0 or 1, and its type stays `integer`;
    /// a host that types by meaning, as the framework does, reads it as a boolean. Empty when the
    /// statement did not prepare.
    pub booleans: Vec<bool>,
}

/// The replayed entry: the host answers [`DDL_QUERY`] (session-scoped), the rest is [`run`].
pub fn analyze(sql: &str, lexed: &Lexed, host: &Host) -> Lazy<Analysis> {
    let rows = host.query(DDL_QUERY, Vec::new(), Scope::Session)?;
    let ddl: Vec<String> = match rows {
        Ok(rows) => rows.iter().filter_map(|r| r.first().and_then(as_str)).map(String::from).collect(),
        Err(e) => return Some(crate::postgres::bare(sql, lexed, Dialect::Sqlite, vec![diag(format!("Cannot read the schema: {}", e.message), 0, 1, 5)])),
    };
    Some(run(sql, lexed, &ddl, &Extras::default()).analysis)
}

/// The direct entry, for a host with a connection of its own: `ddl` is the `sql` column of
/// [`DDL_QUERY`] run there, `extras` the functions it registers there. No round trip; a lex
/// error, a prepare error (SQLite's own message, at its offset) and a replay failure come back as
/// diagnostics, as on the replayed path.
pub fn analyze_ddl(sql: &str, ddl: &[String], extras: &Extras) -> SqliteAnalysis {
    match crate::postgres::lexed(sql, Dialect::Sqlite) {
        Ok(lexed) => run(sql, &lexed, ddl, extras),
        Err(analysis) => SqliteAnalysis { analysis, declared: Vec::new(), readonly: false, unique_keys: HashMap::new(), booleans: Vec::new() },
    }
}

/// The statements the SQLite analysis types: what a `.sql` query file holds, and never a migration.
const KINDS: &[&str] = &["select", "with", "values", "insert", "replace", "update", "delete"];

fn run(sql: &str, lexed: &Lexed, ddl: &[String], extras: &Extras) -> SqliteAnalysis {
    let failed = |analysis: Analysis| SqliteAnalysis { analysis, declared: Vec::new(), readonly: false, unique_keys: HashMap::new(), booleans: Vec::new() };
    if let Err(message) = lex::one_statement(sql, lexed, KINDS) {
        return failed(crate::postgres::bare(sql, lexed, Dialect::Sqlite, vec![diag(message, 0, sql.trim_end().len(), 10)]));
    }
    // SQLite prepares the `?n` rewrite, not the author's text: its own `:name` ends at the first
    // dot, so `:auth.user_id` survives only as a number. `?n` is the lexer's n-th parameter, which
    // is how `params` matches them back, and an error offset maps back through the rewrite.
    let numbered = lex::rewrite(sql, lexed, Target::Numbered);
    let functions: Vec<(&str, i32)> = extras.total.iter().map(|f| (f.name.as_str(), f.arity)).collect();
    // Triggers are not replayed (a body may call a function only the application registers);
    // their headers say which statements a trigger can skip a row of.
    let (triggers, replay): (Vec<String>, Vec<String>) = ddl.iter().cloned().partition(|s| trigger_header(s).is_some());
    let skipping = skipping_triggers(&triggers);
    let t = match tree::analyze(&replay, &numbered.sql, &functions) {
        Ok(t) => t,
        Err(message) => return failed(crate::postgres::bare(sql, lexed, Dialect::Sqlite, vec![diag(message, 0, sql.trim_end().len(), 6)])),
    };
    let mut diagnostics = Vec::new();
    if let Some(err) = &t.error {
        let start = usize::try_from(err.offset).map_or(0, |o| numbered.to_original(o)).min(sql.len());
        diagnostics.push(diag(err.message.clone(), start, if err.offset >= 0 { token_len(&sql[start..]) } else { sql.trim_end().len() }, 6));
        // A statement that fails against the replayed schema often fails because a replay step did.
        for e in &t.schema_errors {
            let message = match &e.function {
                Some(name) => format!("The schema replay could not register the host function {name}: {}", e.message),
                None => format!("The schema replay skipped statement {}: {}", e.index + 1, e.message),
            };
            diagnostics.push(diag(message, 0, 1, 8));
        }
        return failed(crate::postgres::bare(sql, lexed, Dialect::Sqlite, diagnostics));
    }
    let a = Analyzer { tree: &t, extras, skipping: &skipping, nullable_params: guarded_params(&t), level: std::cell::RefCell::new(Level::default()) };
    let (columns, kind) = a.columns();
    let booleans = a.booleans();
    let params = a.params(lexed);
    let (cardinality, reason) = a.cardinality();
    let analysis = Analysis {
        kind,
        params,
        columns,
        cardinality,
        cardinality_reason: reason,
        rewritten: lex::rewrite(sql, lexed, Target::Question),
        style: lexed.style,
        diagnostics,
        unresolved: Vec::new(),
    };
    let unique_keys = t.tables.iter().map(|(name, info)| (name.clone(), info.unique_keys.clone())).collect();
    SqliteAnalysis { analysis, declared: t.columns.iter().map(|c| c.declared.clone()).collect(), readonly: t.readonly, unique_keys, booleans }
}

fn diag(message: impl Into<String>, start: usize, length: usize, code: u32) -> Diagnostic {
    Diagnostic { message: message.into(), start, length: length.max(1), code }
}

fn token_len(rest: &str) -> usize {
    rest.trim_start().chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == ':' || *c == '$' || *c == '@' || *c == '?').count().max(1)
}

/// What an expression is: its affinity as the type model, and whether it can be NULL.
#[derive(Debug, Clone)]
struct Fact {
    ty: Type,
    non_null: bool,
}

impl Fact {
    fn new(aff: &str, non_null: bool) -> Fact {
        Fact { ty: affinity_type(aff), non_null }
    }
    fn unknown() -> Fact {
        Fact { ty: Type::Unknown, non_null: false }
    }
}

/// The type of a table column's values: its affinity, except that a column declared with no type,
/// or `ANY` in a STRICT table, holds a value of any storage class. (`ANY` in a table that is not
/// STRICT is a declared type like any other, with NUMERIC affinity.)
fn column_type(declared: &str, aff: &str, strict: bool) -> Type {
    let declared = declared.trim();
    if declared.is_empty() || (strict && declared.eq_ignore_ascii_case("any")) {
        Type::Unknown
    } else {
        affinity_type(aff)
    }
}

/// An affinity name as the type model; `none` is unknown.
fn affinity_type(aff: &str) -> Type {
    match aff {
        "integer" | "real" | "text" | "blob" | "numeric" => Type::Scalar { name: aff.to_string() },
        _ => Type::Unknown,
    }
}

fn scalar_name(t: &Type) -> Option<&str> {
    match t {
        Type::Scalar { name } => Some(name.as_str()),
        _ => None,
    }
}

fn is_numeric(t: &Type) -> bool {
    matches!(scalar_name(t), Some("integer" | "real" | "numeric"))
}

/// The type several arms share: one type when they agree, `real` for a mix of numbers, unknown otherwise.
fn shared(facts: &[&Fact]) -> Type {
    let known: Vec<&Type> = facts.iter().map(|f| &f.ty).filter(|t| !matches!(t, Type::Unknown)).collect();
    let Some(first) = known.first() else { return Type::Unknown };
    if known.iter().all(|t| t == first) {
        return (*first).clone();
    }
    if known.iter().all(|t| is_numeric(t)) {
        return Type::Scalar { name: "real".into() };
    }
    Type::Unknown
}

/// The FROM lists in scope, innermost last: a column reference names a cursor of one of them.
type ScopeStack<'a> = Vec<&'a [From]>;

struct Analyzer<'a> {
    tree: &'a Tree,
    extras: &'a Extras,
    /// Per table (lowercase), the events (`TRIGGER_ROW_INSERT`, `_UPDATE`, `_DELETE`) a BEFORE or
    /// INSTEAD OF trigger fires on: its `SELECT RAISE(IGNORE)` skips the row.
    skipping: &'a HashMap<String, i32>,
    /// The parameters (`?n`) the caller may pass NULL for, because the statement tests them for
    /// NULL or coalesces them ([`guarded_params`]); every other one is a non-null value, as on
    /// Postgres.
    nullable_params: HashSet<String>,
    /// What the WHERE and the grouping of the SELECT arm whose columns are being typed prove.
    level: std::cell::RefCell<Level>,
}

/// What one SELECT arm's WHERE and grouping prove of the expressions in its select list.
#[derive(Debug, Clone, Default)]
struct Level {
    /// Columns (cursor, name) a strict WHERE test rejects NULL in: not NULL in a returned row.
    narrowed: HashSet<(i64, String)>,
    /// Cursors of outer-joined items a strict WHERE test reads: a returned row has their row.
    reduced: HashSet<i64>,
    /// A plain GROUP BY: no group is empty.
    grouped: bool,
}

/// The parameters the statement tests for NULL (`?1 IS NULL`, `?1 IS NOT NULL`) or passes first to
/// `coalesce` or `ifnull`: the caller means to pass NULL for them.
fn guarded_params(t: &Tree) -> HashSet<String> {
    fn walk(n: &Node, out: &mut HashSet<String>) {
        let variable = |x: Option<&Node>| x.filter(|x| x.op == "variable").and_then(|x| x.name.clone());
        match n.op.as_str() {
            "isnull" | "notnull" | "is" | "is not" => {
                out.extend(variable(n.left.as_deref()));
                out.extend(variable(n.right.as_deref()));
            }
            "function" if matches!(n.name.as_deref(), Some("coalesce" | "ifnull")) => out.extend(variable(n.args.first())),
            _ => {}
        }
        for child in n.left.iter().chain(n.right.iter()).map(|b| b.as_ref()).chain(n.list.iter()).chain(n.args.iter()) {
            walk(child, out);
        }
        if let Some(sel) = &n.select {
            walk_select(sel, out);
        }
    }
    fn walk_select(sel: &Sel, out: &mut HashSet<String>) {
        for c in &sel.columns {
            walk(&c.expr, out);
        }
        for n in sel.where_.iter().chain(sel.having.iter()).chain(sel.group_by.iter()).chain(sel.order_by.iter()).chain(sel.limit.iter()).chain(sel.offset.iter()) {
            walk(n, out);
        }
        for f in &sel.from {
            if let Some(sub) = &f.subquery {
                walk_select(sub, out);
            }
        }
        if let Some(prior) = &sel.prior {
            walk_select(prior, out);
        }
    }
    let mut out = HashSet::new();
    if let Some(sel) = &t.statement {
        walk_select(sel, &mut out);
    }
    if let Some(dml) = &t.dml {
        for n in dml.values.iter().flatten().chain(dml.set.iter().map(|s| &s.expr)).chain(dml.where_.iter()) {
            walk(n, &mut out);
        }
        if let Some(u) = &dml.upsert {
            for n in u.set.iter().map(|s| &s.expr).chain(u.where_.iter()) {
                walk(n, &mut out);
            }
        }
    }
    for c in t.returning.iter().flatten() {
        walk(&c.expr, &mut out);
    }
    out
}

/// What a SELECT arm's WHERE proves of its own FROM items, by the strict tests at its top level
/// (not the ON terms of outer joins SQLite moved there), and whether it groups plainly.
fn level_of(sel: &Sel, extras: &Extras) -> Level {
    let own: HashSet<i64> = sel.from.iter().map(|f| f.cursor).collect();
    let outer: HashSet<i64> = sel.from.iter().filter(|f| f.outer).map(|f| f.cursor).collect();
    let mut narrowed = HashSet::new();
    for term in conjuncts(sel.where_.as_ref()).into_iter().filter(|t| !t.outer_on) {
        narrowed.extend(strict_in(term, extras).into_iter().filter(|(cursor, _)| own.contains(cursor)));
    }
    let reduced = narrowed.iter().map(|(cursor, _)| *cursor).filter(|c| outer.contains(c)).collect();
    Level { narrowed, reduced, grouped: !sel.group_by.is_empty() }
}

/// The columns a WHERE test at the top level rejects NULL in: under OR the ones every arm does; a
/// comparison, `IN`, `BETWEEN`, `IS NOT NULL`, `LIKE` or `GLOB`, in the columns its operands
/// answer NULL for. `IS` and `IS NOT` compare NULL like a value, and reject nothing. A function the
/// host registers over a built-in's name is the one a call resolves to, and proves nothing.
fn strict_in(n: &Node, extras: &Extras) -> HashSet<(i64, String)> {
    let nulls = |x: &Node| nulls_with(x, extras);
    match n.op.as_str() {
        "and" => n.left.iter().chain(n.right.iter()).flat_map(|x| strict_in(x, extras)).collect(),
        "or" => {
            let (Some(l), Some(r)) = (n.left.as_deref(), n.right.as_deref()) else { return HashSet::new() };
            let right = strict_in(r, extras);
            strict_in(l, extras).into_iter().filter(|c| right.contains(c)).collect()
        }
        "notnull" | "in" | "between" => n.left.as_deref().map(nulls).unwrap_or_default(),
        "=" | "!=" | "<" | "<=" | ">" | ">=" => n.left.iter().chain(n.right.iter()).flat_map(|x| nulls(x)).collect(),
        "not" => match n.left.as_deref() {
            Some(inner) if inner.op == "isnull" => inner.left.as_deref().map(nulls).unwrap_or_default(),
            Some(inner) => strict_in(inner, extras).into_iter().filter(|_| matches!(inner.op.as_str(), "=" | "!=" | "<" | "<=" | ">" | ">=" | "in" | "between")).collect(),
            None => HashSet::new(),
        },
        "function" if !n.agg && !n.window && matches!(n.name.as_deref(), Some("like" | "glob")) && extras.function(n.name.as_deref().unwrap_or(""), n.args.len()).is_none() => {
            n.args.iter().flat_map(nulls).collect()
        }
        "column" => nulls(n),
        _ => HashSet::new(),
    }
}

/// The columns an expression answers NULL for: the column itself, and through arithmetic,
/// concatenation, a cast, a COLLATE or a built-in that is NULL for a NULL argument (not one the
/// host registers over it), every column an operand answers NULL for.
fn nulls_with(n: &Node, extras: &Extras) -> HashSet<(i64, String)> {
    const STRICT: &[&str] = &["upper", "lower", "length", "abs", "trim", "ltrim", "rtrim", "substr", "substring", "replace", "round", "instr", "unicode", "sign", "octet_length"];
    match n.op.as_str() {
        "column" => n.column.clone().map(|c| HashSet::from([(n.cursor, c)])).unwrap_or_default(),
        "+" | "-" | "*" | "/" | "%" | "||" => n.left.iter().chain(n.right.iter()).flat_map(|x| nulls_with(x, extras)).collect(),
        "neg" | "pos" | "cast" | "collate" => n.left.as_deref().map(|x| nulls_with(x, extras)).unwrap_or_default(),
        "function" if !n.agg && !n.window && n.name.as_deref().is_some_and(|f| STRICT.contains(&f) && extras.function(f, n.args.len()).is_none()) => n.args.iter().flat_map(|x| nulls_with(x, extras)).collect(),
        _ => HashSet::new(),
    }
}

/// A `CREATE TRIGGER` statement's timing (`before`, `after`, `instead`), event (`insert`,
/// `update`, `delete`) and table, lowercase and unquoted; `None` for any other statement.
fn trigger_header(sql: &str) -> Option<(String, String, String)> {
    let words: Vec<String> = sql
        .split(|c: char| c.is_whitespace() || c == '(')
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches(|c| matches!(c, '"' | '`' | '[' | ']' | '\'')).to_ascii_lowercase())
        .collect();
    // Steps over the next word when it is one of `want`.
    fn take(words: &[String], i: &mut usize, want: &[&str]) -> bool {
        let hit = words.get(*i).is_some_and(|w| want.contains(&w.as_str()));
        if hit {
            *i += 1;
        }
        hit
    }
    let mut i = 0;
    if !take(&words, &mut i, &["create"]) {
        return None;
    }
    take(&words, &mut i, &["temp", "temporary"]);
    if !take(&words, &mut i, &["trigger"]) {
        return None;
    }
    if take(&words, &mut i, &["if"]) {
        take(&words, &mut i, &["not"]);
        take(&words, &mut i, &["exists"]);
    }
    i += 1; // the trigger's name
    let timing = if take(&words, &mut i, &["before"]) {
        "before"
    } else if take(&words, &mut i, &["after"]) {
        "after"
    } else if take(&words, &mut i, &["instead"]) {
        take(&words, &mut i, &["of"]);
        "instead"
    } else {
        // "If no timing is specified, BEFORE is the default."
        "before"
    };
    let event = words.get(i)?.clone();
    let on = words.iter().skip(i).position(|w| w == "on")? + i;
    let table = words.get(on + 1)?.rsplit('.').next()?.to_string();
    Some((timing.to_string(), event, table))
}

/// The events per table that some BEFORE or INSTEAD OF trigger fires on.
fn skipping_triggers(triggers: &[String]) -> HashMap<String, i32> {
    use crate::cardinality::{TRIGGER_ROW_DELETE, TRIGGER_ROW_INSERT, TRIGGER_ROW_UPDATE};
    let mut out: HashMap<String, i32> = HashMap::new();
    for sql in triggers {
        let Some((timing, event, table)) = trigger_header(sql) else { continue };
        if timing == "after" {
            continue;
        }
        let bit = match event.as_str() {
            "insert" => TRIGGER_ROW_INSERT,
            "update" => TRIGGER_ROW_UPDATE,
            "delete" => TRIGGER_ROW_DELETE,
            _ => TRIGGER_ROW_INSERT | TRIGGER_ROW_UPDATE | TRIGGER_ROW_DELETE,
        };
        *out.entry(table).or_default() |= bit;
    }
    out
}

/// When a function's result can be NULL.
#[derive(Clone, Copy)]
enum Nulls {
    /// Never, whatever the arguments.
    Never,
    /// Only when an argument can be.
    IfArgs,
    /// Always possibly (an aggregate over no rows, a lookup that can miss).
    Always,
}

impl Analyzer<'_> {
    // ---- result columns ---------------------------------------------------------------------

    fn columns(&self) -> (Vec<Column>, String) {
        let t = self.tree;
        let kind = t.dml.as_ref().map(|d| if d.kind == "upsert" { "insert".to_string() } else { d.kind.clone() }).unwrap_or_else(|| if t.statement.is_some() { "select".into() } else { String::new() });
        let facts: Vec<Fact> = if let Some(returning) = &t.returning {
            returning.iter().map(|c| self.type_of(&c.expr, &Vec::new())).collect()
        } else if let (Some(sel), None) = (&t.statement, &t.dml) {
            (0..sel.columns.len()).map(|i| self.select_column(sel, i, &Vec::new())).collect()
        } else {
            Vec::new()
        };
        let columns = t
            .columns
            .iter()
            .enumerate()
            .map(|(i, public)| {
                let fact = facts.get(i).cloned().unwrap_or_else(Fact::unknown);
                // A column SQLite could not type but names from a table: its declared affinity. The
                // fact for a column declared without a type, or `ANY`, is already unknown on purpose.
                let fact = match (&fact.ty, &public.declared) {
                    (Type::Unknown, Some(declared)) => Fact { ty: column_type(declared, affinity(declared), true), non_null: fact.non_null },
                    _ => fact,
                };
                let type_name = scalar_name(&fact.ty).unwrap_or("").to_string();
                Column { name: public.name.clone(), ty: fact.ty, nullable: !fact.non_null, type_name }
            })
            .collect();
        (columns, kind)
    }

    /// Per result column, whether it is a truth value ([`SqliteAnalysis::booleans`]).
    fn booleans(&self) -> Vec<bool> {
        let t = self.tree;
        let truths: Vec<bool> = if let Some(returning) = &t.returning {
            returning.iter().map(|c| self.truth(&c.expr, &Vec::new())).collect()
        } else if let (Some(sel), None) = (&t.statement, &t.dml) {
            (0..sel.columns.len()).map(|i| self.select_truth(sel, i, &Vec::new())).collect()
        } else {
            Vec::new()
        };
        (0..t.columns.len()).map(|i| truths.get(i).copied().unwrap_or(false)).collect()
    }

    /// Whether column `i` of a select is a truth value in every arm of a compound.
    fn select_truth(&self, sel: &Sel, i: usize, scope: &ScopeStack) -> bool {
        let mut s = Some(sel);
        while let Some(arm) = s {
            let mut inner = scope.clone();
            inner.push(&arm.from);
            if !arm.columns.get(i).is_some_and(|c| self.truth(&c.expr, &inner)) {
                return false;
            }
            s = arm.prior.as_deref();
        }
        true
    }

    /// Whether an expression is a truth value: what a comparison, a test or a logical operator
    /// answers, and what passes one through unchanged. A NULL arm is no counterexample (`CASE`
    /// without `ELSE`), and nullability is [`Analyzer::type_of`]'s business.
    fn truth(&self, n: &Node, scope: &ScopeStack) -> bool {
        let every = |nodes: &mut dyn Iterator<Item = &Node>| {
            let mut any = false;
            for x in nodes {
                if x.op == "null" {
                    continue;
                }
                if !self.truth(x, scope) {
                    return false;
                }
                any = true;
            }
            any
        };
        match n.op.as_str() {
            "=" | "!=" | "<" | "<=" | ">" | ">=" | "is" | "is not" | "isnull" | "notnull" | "in" | "between" | "exists" | "and" | "or" | "not" | "truefalse" => true,
            "collate" => n.left.as_deref().is_some_and(|l| self.truth(l, scope)),
            "function" if !n.agg && !n.window => match n.name.as_deref().unwrap_or("") {
                "like" | "glob" | "regexp" | "match" => self.extras.function(n.name.as_deref().unwrap_or(""), n.args.len()).is_none(),
                "likely" | "unlikely" | "likelihood" => n.args.first().is_some_and(|a| self.truth(a, scope)),
                "coalesce" | "ifnull" => every(&mut n.args.iter()),
                "iif" | "if" => every(&mut n.args.iter().skip(1)),
                _ => false,
            },
            // `list` alternates WHEN and THEN; an odd tail is the ELSE.
            "case" => {
                let results = n.list.chunks(2).filter_map(|pair| if pair.len() == 2 { Some(&pair[1]) } else { None });
                let otherwise = if n.list.len() % 2 == 1 { n.list.last() } else { None };
                every(&mut results.chain(otherwise))
            }
            "select" => n.select.as_deref().is_some_and(|sub| self.select_truth(sub, 0, scope)),
            "column" => {
                for froms in scope.iter().rev() {
                    if let Some(f) = froms.iter().find(|f| f.cursor == n.cursor) {
                        return f.subquery.as_deref().is_some_and(|sub| self.select_truth(sub, n.index.max(0) as usize, scope));
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Column `i` of a select: for a compound, the leftmost arm's type and any arm's nullability.
    fn select_column(&self, sel: &Sel, i: usize, scope: &ScopeStack) -> Fact {
        let mut arms: Vec<&Sel> = Vec::new();
        let mut s = Some(sel);
        while let Some(arm) = s {
            arms.push(arm);
            s = arm.prior.as_deref();
        }
        arms.reverse();
        let facts: Vec<Fact> = arms
            .iter()
            .map(|arm| {
                let mut inner = scope.clone();
                inner.push(&arm.from);
                let outer = self.level.replace(level_of(arm, self.extras));
                let fact = arm.columns.get(i).map(|c| self.type_of(&c.expr, &inner)).unwrap_or_else(Fact::unknown);
                self.level.replace(outer);
                fact
            })
            .collect();
        let Some(first) = facts.first() else { return Fact::unknown() };
        let ty = if facts.iter().all(|f| f.ty == first.ty) { first.ty.clone() } else { shared(&facts.iter().collect::<Vec<_>>()) };
        Fact { ty, non_null: facts.iter().all(|f| f.non_null) }
    }

    /// A column reference: through the cursor to a subquery's column, or the table column itself.
    fn column_ref(&self, n: &Node, scope: &ScopeStack) -> Fact {
        // A strict WHERE test on the column, or on any column of its outer-joined item, keeps
        // its NULL (or the item's null extension) out of every returned row.
        let (narrowed, reduced) = {
            let level = self.level.borrow();
            (n.column.as_ref().is_some_and(|c| level.narrowed.contains(&(n.cursor, c.clone()))), level.reduced.contains(&n.cursor))
        };
        for froms in scope.iter().rev() {
            if let Some(f) = froms.iter().find(|f| f.cursor == n.cursor) {
                if let Some(sub) = &f.subquery {
                    let inner = self.select_column(sub, n.index.max(0) as usize, scope);
                    return Fact { ty: inner.ty, non_null: narrowed || (inner.non_null && ((!f.outer && !n.outer) || reduced)) };
                }
                break;
            }
        }
        let aff = n.affinity.as_deref().unwrap_or(&n.aff);
        let ty = match &n.declared {
            Some(declared) if !n.rowid => column_type(declared, aff, n.strict),
            _ => affinity_type(aff),
        };
        Fact { ty, non_null: narrowed || ((n.not_null || n.rowid) && (!n.outer || reduced)) }
    }

    // ---- expressions -------------------------------------------------------------------------

    fn type_of(&self, n: &Node, scope: &ScopeStack) -> Fact {
        let arg = |x: &Option<Box<Node>>| x.as_deref().map(|x| self.type_of(x, scope)).unwrap_or_else(Fact::unknown);
        match n.op.as_str() {
            "column" => self.column_ref(n, scope),
            "integer" | "truefalse" => Fact::new("integer", true),
            "float" => Fact::new("real", true),
            "string" => Fact::new("text", true),
            "blob" => Fact::new("blob", true),
            "null" => Fact::unknown(),
            // A parameter is a non-null value unless the statement means to take NULL for it.
            "variable" => Fact { ty: Type::Unknown, non_null: n.name.as_ref().is_some_and(|name| !self.nullable_params.contains(name)) },
            "function" => {
                let args: Vec<Fact> = n.args.iter().map(|a| self.type_of(a, scope)).collect();
                self.function(n, &args)
            }
            "+" | "-" | "*" => {
                let (l, r) = (arg(&n.left), arg(&n.right));
                Fact { ty: arithmetic(&l.ty, &r.ty), non_null: l.non_null && r.non_null }
            }
            "/" | "%" => {
                // A zero divisor gives NULL; a literal divisor is one the statement rules out.
                let (l, r) = (arg(&n.left), arg(&n.right));
                let literal_divisor = n.right.as_deref().is_some_and(|d| matches!(d.op.as_str(), "integer" | "float") && d.value.as_deref().and_then(|v| v.parse::<f64>().ok()).is_some_and(|v| v != 0.0));
                Fact { ty: arithmetic(&l.ty, &r.ty), non_null: l.non_null && r.non_null && literal_divisor }
            }
            "||" => {
                let (l, r) = (arg(&n.left), arg(&n.right));
                Fact::new("text", l.non_null && r.non_null)
            }
            "=" | "!=" | "<" | "<=" | ">" | ">=" | "and" | "or" | "in" | "between" | "&" | "|" | "<<" | ">>" => {
                let (l, r) = (arg(&n.left), arg(&n.right));
                let list_non_null = n.list.iter().all(|x| self.type_of(x, scope).non_null);
                let right_non_null = if n.right.is_some() { r.non_null } else { list_non_null && n.select.is_none() };
                Fact::new("integer", l.non_null && right_non_null)
            }
            "is" | "is not" | "isnull" | "notnull" | "exists" => Fact::new("integer", true),
            "not" | "~" => {
                let l = arg(&n.left);
                Fact::new("integer", l.non_null)
            }
            "neg" => {
                let l = arg(&n.left);
                Fact { ty: if is_numeric(&l.ty) { l.ty } else { Type::Scalar { name: "real".into() } }, non_null: l.non_null }
            }
            // Unary plus does nothing to any storage class: `+name` is the text 'bob'. SQLite's
            // parser also writes a one-element `x IN (e)` as `x = +e`.
            "pos" => arg(&n.left),
            "cast" => {
                let l = arg(&n.left);
                Fact::new(n.affinity.as_deref().unwrap_or("none"), l.non_null)
            }
            "collate" => arg(&n.left),
            "case" => {
                // `list` alternates WHEN and THEN; an odd tail is the ELSE.
                let results: Vec<Fact> = n.list.chunks(2).filter_map(|pair| if pair.len() == 2 { Some(self.type_of(&pair[1], scope)) } else { None }).collect();
                let otherwise = if n.list.len() % 2 == 1 { n.list.last().map(|e| self.type_of(e, scope)) } else { None };
                let mut all: Vec<&Fact> = results.iter().collect();
                if let Some(o) = &otherwise {
                    all.push(o);
                }
                Fact { ty: shared(&all), non_null: otherwise.is_some() && all.iter().all(|f| f.non_null) }
            }
            "select" => {
                // A scalar subquery: its first column, NULL when it finds no row, unless it always
                // finds exactly one (an ungrouped aggregate: `(SELECT count(*) FROM t)`). Its
                // column is then its own answer, so `(SELECT max(x) FROM t)` stays nullable.
                let Some(sub) = n.select.as_deref() else { return Fact::unknown() };
                let inner = self.select_column(sub, 0, scope);
                Fact { ty: inner.ty, non_null: inner.non_null && self.exactly_one(sub).is_ok() }
            }
            _ => Fact::unknown(),
        }
    }

    /// The built-in functions: their result type, and when it is NULL.
    fn function(&self, n: &Node, args: &[Fact]) -> Fact {
        let name = n.name.as_deref().unwrap_or("");
        let all_non_null = args.iter().all(|a| a.non_null);
        let first = args.first().cloned().unwrap_or_else(Fact::unknown);
        let done = |aff: &str, nulls: Nulls| {
            let non_null = match nulls {
                Nulls::Never => true,
                Nulls::IfArgs => all_non_null,
                Nulls::Always => false,
            };
            Fact::new(aff, non_null)
        };
        let typed = |ty: Type, nulls: Nulls| Fact { ty, non_null: matches!(nulls, Nulls::Never) || (matches!(nulls, Nulls::IfArgs) && all_non_null) };
        if !n.agg && !n.window {
            if let Some(f) = self.extras.function(name, args.len()) {
                return done(oid_affinity(f.returns), Nulls::IfArgs);
            }
        }
        if n.window {
            return match name {
                "row_number" | "rank" | "dense_rank" | "ntile" => done("integer", Nulls::Never),
                "percent_rank" | "cume_dist" => done("real", Nulls::Never),
                "count" => done("integer", Nulls::Never),
                "total" => done("real", Nulls::Never),
                "sum" => typed(if is_numeric(&first.ty) { first.ty.clone() } else { Type::Scalar { name: "real".into() } }, Nulls::Always),
                "avg" => done("real", Nulls::Always),
                "min" | "max" | "lag" | "lead" | "first_value" | "last_value" | "nth_value" => typed(first.ty.clone(), Nulls::Always),
                "group_concat" | "string_agg" => done("text", Nulls::Always),
                _ => Fact::unknown(),
            };
        }
        if n.agg {
            // Under a plain GROUP BY every group has a row, so an aggregate that is NULL only
            // over no rows or only NULLs follows its argument. (A FILTER makes the call a window
            // call in SQLite's tree, and is read above.) `sum` stays nullable: a sum of `1e999`
            // and `-1e999` is NaN, which SQLite stores as NULL.
            let grouped = self.level.borrow().grouped;
            return match name {
                "count" => done("integer", Nulls::Never),
                "total" => done("real", Nulls::Never),
                "sum" => typed(if is_numeric(&first.ty) { first.ty.clone() } else { Type::Scalar { name: "real".into() } }, Nulls::Always),
                "avg" => done("real", Nulls::Always),
                "min" | "max" => typed(first.ty.clone(), if grouped { Nulls::IfArgs } else { Nulls::Always }),
                "group_concat" | "string_agg" => done("text", if grouped { Nulls::IfArgs } else { Nulls::Always }),
                "json_group_array" | "jsonb_group_array" | "json_group_object" | "jsonb_group_object" => done("text", Nulls::Never),
                _ => Fact::unknown(),
            };
        }
        match name {
            "coalesce" | "ifnull" => Fact { ty: shared(&args.iter().collect::<Vec<_>>()), non_null: args.iter().any(|a| a.non_null) },
            "nullif" => Fact { ty: first.ty.clone(), non_null: false },
            "iif" | "if" => {
                let arms: Vec<&Fact> = args.iter().skip(1).collect();
                Fact { ty: shared(&arms), non_null: arms.len() >= 2 && arms.iter().all(|a| a.non_null) }
            }
            "min" | "max" => Fact { ty: shared(&args.iter().collect::<Vec<_>>()), non_null: all_non_null },
            "abs" => typed(if is_numeric(&first.ty) { first.ty.clone() } else { Type::Scalar { name: "real".into() } }, Nulls::IfArgs),
            "round" | "julianday" | "timediff_seconds" => done("real", Nulls::IfArgs),
            "length" | "instr" | "unicode" | "unixepoch" | "json_array_length" | "json_valid" | "octet_length" | "sign" => done("integer", Nulls::IfArgs),
            "like" | "glob" | "regexp" | "match" | "likely" | "unlikely" | "likelihood" => {
                if matches!(name, "likely" | "unlikely" | "likelihood") { first.clone() } else { done("integer", Nulls::IfArgs) }
            }
            // `concat` skips NULL arguments (all NULL is ''), and `concat_ws` too, NULL only for a
            // NULL separator.
            "concat" => done("text", Nulls::Never),
            "concat_ws" => Fact::new("text", first.non_null),
            "upper" | "lower" | "trim" | "ltrim" | "rtrim" | "substr" | "substring" | "replace" | "char" | "printf" | "format"
            | "date" | "time" | "datetime" | "strftime" | "timediff" | "json" | "jsonb" | "json_array" | "json_object" | "json_insert" | "json_replace" | "json_set"
            | "json_remove" | "json_patch" | "json_quote" | "json_pretty" | "json_type" => done("text", Nulls::IfArgs),
            "hex" | "quote" | "typeof" => done("text", Nulls::Never),
            "sqlite_version" | "sqlite_source_id" => done("text", Nulls::Never),
            "random" | "last_insert_rowid" | "changes" | "total_changes" => done("integer", Nulls::Never),
            "randomblob" | "zeroblob" => done("blob", Nulls::IfArgs),
            "json_extract" | "jsonb_extract" | "->" | "->>" => Fact::unknown(),
            "acos" | "acosh" | "asin" | "asinh" | "atan" | "atan2" | "atanh" | "ceil" | "ceiling" | "cos" | "cosh" | "degrees" | "exp" | "floor" | "ln" | "log"
            | "log10" | "log2" | "mod" | "pow" | "power" | "radians" | "sin" | "sinh" | "sqrt" | "tan" | "tanh" | "trunc" => done("real", Nulls::IfArgs),
            "pi" => done("real", Nulls::Never),
            _ => Fact::unknown(),
        }
    }

    // ---- parameters --------------------------------------------------------------------------

    /// Every parameter, typed by the context SQLite placed it in; the first typed context wins.
    fn params(&self, lexed: &Lexed) -> Vec<Param> {
        let mut found: Vec<(String, Fact)> = Vec::new();
        let t = self.tree;
        if let Some(sel) = &t.statement {
            let targets: Vec<Option<Fact>> = match &t.dml {
                // The SELECT that feeds an INSERT: its columns type from the target columns.
                Some(dml) if dml.kind == "insert" => dml.columns.iter().map(|c| c.as_deref().and_then(|c| self.table_column(&dml.table, c))).collect(),
                _ => Vec::new(),
            };
            self.walk_select(sel, &Vec::new(), &targets, &mut found);
        }
        // A parameter that is the whole value written to a column may be null when that column
        // may, and no NOT NULL column it also fills forbids it.
        let mut fills: HashMap<String, bool> = HashMap::new();
        let mut fill = |v: &Node, target: &Option<Fact>| {
            if let (true, Some(name), Some(target)) = (v.op == "variable", v.name.as_ref(), target) {
                *fills.entry(name.clone()).or_insert(true) &= !target.non_null;
            }
        };
        if let Some(dml) = &t.dml {
            let scope: ScopeStack = Vec::new();
            if let Some(values) = &dml.values {
                for (i, v) in values.iter().enumerate() {
                    let target = dml.columns.get(i).and_then(|c| c.as_deref()).and_then(|c| self.table_column(&dml.table, c));
                    fill(v, &target);
                    self.walk(v, Ctx::Typed(target), &scope, &mut found);
                }
            }
            for item in dml.set.iter().chain(dml.upsert.iter().flat_map(|u| u.set.iter())) {
                let target = item.column.as_deref().and_then(|c| self.table_column(&dml.table, c));
                fill(&item.expr, &target);
                self.walk(&item.expr, Ctx::Typed(target), &scope, &mut found);
            }
            if let Some(w) = &dml.where_ {
                self.walk(w, Ctx::Bool, &scope, &mut found);
            }
            if let Some(w) = dml.upsert.as_ref().and_then(|u| u.where_.as_ref()) {
                self.walk(w, Ctx::Bool, &scope, &mut found);
            }
        }
        if let Some(returning) = &t.returning {
            for c in returning {
                self.walk(&c.expr, Ctx::None, &Vec::new(), &mut found);
            }
        }
        // The tree was prepared over the `?n` rewrite (see `analyze`): SQLite's `?k` is the lexer's
        // k-th parameter, whatever the author wrote.
        lexed
            .params
            .iter()
            .enumerate()
            .map(|(k, name)| {
                let numbered = format!("?{}", k + 1);
                let fact = found
                    .iter()
                    .filter(|(n, _)| *n == numbered)
                    .map(|(_, f)| f)
                    .find(|f| !matches!(f.ty, Type::Unknown))
                    .cloned()
                    .unwrap_or_else(Fact::unknown);
                let type_name = scalar_name(&fact.ty).unwrap_or("").to_string();
                let nullable = fills.get(&numbered).copied().unwrap_or(false) || self.nullable_params.contains(&numbered);
                Param { name: name.clone(), ty: fact.ty, type_name, nullable }
            })
            .collect()
    }

    fn table_column(&self, table: &str, column: &str) -> Option<Fact> {
        let info = self.tree.tables.get(table)?;
        let col = info.columns.iter().find(|c| c.name.eq_ignore_ascii_case(column))?;
        Some(Fact { ty: column_type(&col.declared, &col.affinity, info.strict), non_null: col.not_null || info.rowid_alias.as_deref().is_some_and(|a| a.eq_ignore_ascii_case(column)) })
    }

    fn walk_select(&self, sel: &Sel, scope: &ScopeStack, targets: &[Option<Fact>], found: &mut Vec<(String, Fact)>) {
        // In a compound every arm's column i is one result column: a parameter in one arm takes
        // the type the other arms give it (`SELECT name FROM users UNION ALL SELECT :x`).
        let compound: Vec<Option<Fact>>;
        let targets = if targets.is_empty() && sel.prior.is_some() {
            compound = (0..sel.columns.len())
                .map(|i| {
                    let ty = self.select_column(sel, i, scope).ty;
                    (!matches!(ty, Type::Unknown)).then_some(Fact { ty, non_null: true })
                })
                .collect();
            compound.as_slice()
        } else {
            targets
        };
        let mut inner = scope.clone();
        inner.push(&sel.from);
        for (i, c) in sel.columns.iter().enumerate() {
            self.walk(&c.expr, Ctx::Typed(targets.get(i).cloned().flatten()), &inner, found);
        }
        for f in &sel.from {
            if let Some(sub) = &f.subquery {
                self.walk_select(sub, &inner, &[], found);
            }
            if let Some(on) = &f.on {
                self.walk(on, Ctx::Bool, &inner, found);
            }
        }
        if let Some(w) = &sel.where_ {
            self.walk(w, Ctx::Bool, &inner, found);
        }
        for g in &sel.group_by {
            self.walk(g, Ctx::None, &inner, found);
        }
        if let Some(h) = &sel.having {
            self.walk(h, Ctx::Bool, &inner, found);
        }
        for o in &sel.order_by {
            self.walk(o, Ctx::None, &inner, found);
        }
        for l in sel.limit.iter().chain(sel.offset.iter()) {
            self.walk(l, Ctx::Typed(Some(Fact::new("integer", true))), &inner, found);
        }
        if let Some(prior) = &sel.prior {
            self.walk_select(prior, scope, targets, found);
        }
    }

    /// Visits every node; a variable takes the type its context gives it.
    fn walk(&self, n: &Node, ctx: Ctx, scope: &ScopeStack, found: &mut Vec<(String, Fact)>) {
        if n.op == "variable" {
            let fact = match ctx {
                Ctx::Typed(Some(f)) => f,
                Ctx::Typed(None) | Ctx::None => Fact::unknown(),
                Ctx::Bool => Fact::new("integer", false),
            };
            found.push((n.name.clone().unwrap_or_default(), fact));
            return;
        }
        let sibling = |other: &Option<Box<Node>>| other.as_deref().map(|o| self.type_of(o, scope));
        let child_ctx = |slot: Slot| -> Ctx {
            match n.op.as_str() {
                "=" | "!=" | "<" | "<=" | ">" | ">=" | "is" | "is not" => Ctx::Typed(match slot {
                    Slot::Left => sibling(&n.right),
                    Slot::Right => sibling(&n.left),
                    _ => None,
                }),
                "in" => Ctx::Typed(match slot {
                    Slot::Left => n.list.first().map(|x| self.type_of(x, scope)).or_else(|| n.select.as_deref().map(|s| self.select_column(s, 0, scope))),
                    Slot::List(_) => sibling(&n.left),
                    _ => None,
                }),
                "between" => Ctx::Typed(match slot {
                    Slot::Left => n.list.first().map(|x| self.type_of(x, scope)),
                    Slot::List(_) => sibling(&n.left),
                    _ => None,
                }),
                "cast" => Ctx::Typed(Some(Fact::new(n.affinity.as_deref().unwrap_or("none"), true))),
                "+" | "-" | "*" | "/" | "%" => {
                    let other = match slot {
                        Slot::Left => sibling(&n.right),
                        _ => sibling(&n.left),
                    };
                    Ctx::Typed(Some(match other {
                        Some(f) if is_numeric(&f.ty) => f,
                        _ => Fact::new("real", true),
                    }))
                }
                "||" => Ctx::Typed(Some(Fact::new("text", true))),
                "neg" | "~" | "&" | "|" | "<<" | ">>" => Ctx::Typed(Some(Fact::new("real", true))),
                // Unary plus hands its operand the context it stands in: `x IN (?)` is `x = +?`.
                "pos" => ctx.clone(),
                "and" | "or" | "not" => Ctx::Bool,
                "case" => match slot {
                    // A WHEN compares with the base when there is one, else it is a condition; a
                    // THEN or ELSE shares the other results' type, or failing that the type the
                    // CASE stands in (`name = CASE WHEN … THEN :a ELSE :b END`).
                    Slot::List(i) if i % 2 == 0 && i + 1 < n.list.len() => match &n.left {
                        Some(base) => Ctx::Typed(Some(self.type_of(base, scope))),
                        None => Ctx::Bool,
                    },
                    Slot::List(_) => {
                        let others: Vec<Fact> = n.list.iter().enumerate().filter(|(j, _)| j % 2 == 1 || (*j == n.list.len() - 1 && n.list.len() % 2 == 1)).map(|(_, x)| self.type_of(x, scope)).collect();
                        let refs: Vec<&Fact> = others.iter().collect();
                        let ty = shared(&refs);
                        match ty {
                            Type::Unknown => outer_type(&ctx),
                            ty => Ctx::Typed(Some(Fact { ty, non_null: true })),
                        }
                    }
                    // A base compares with each WHEN value (`CASE :kind WHEN 'a' THEN …`).
                    Slot::Left => {
                        let values: Vec<Fact> = n.list.iter().enumerate().filter(|(j, _)| j % 2 == 0 && j + 1 < n.list.len()).map(|(_, x)| self.type_of(x, scope)).collect();
                        let refs: Vec<&Fact> = values.iter().collect();
                        match shared(&refs) {
                            Type::Unknown => Ctx::None,
                            ty => Ctx::Typed(Some(Fact { ty, non_null: true })),
                        }
                    }
                    _ => Ctx::None,
                },
                // A function whose result is one of its arguments gives an argument its siblings'
                // type, or failing that the type the call stands in (`name = coalesce(:a, :b)`).
                "function" => match (slot, self.argument(n, i_of(slot), scope)) {
                    (Slot::Arg(_), Ctx::Typed(None) | Ctx::None) if matches!(n.name.as_deref(), Some("coalesce" | "ifnull" | "nullif" | "min" | "max")) && !n.agg => outer_type(&ctx),
                    (Slot::Arg(i), Ctx::Typed(None) | Ctx::None) if i > 0 && matches!(n.name.as_deref(), Some("iif" | "if")) => outer_type(&ctx),
                    (Slot::Arg(_), c) => c,
                    _ => Ctx::None,
                },
                _ => Ctx::None,
            }
        };
        if let Some(l) = &n.left {
            self.walk(l, child_ctx(Slot::Left), scope, found);
        }
        if let Some(r) = &n.right {
            self.walk(r, child_ctx(Slot::Right), scope, found);
        }
        for (i, x) in n.list.iter().enumerate() {
            self.walk(x, child_ctx(Slot::List(i)), scope, found);
        }
        for (i, x) in n.args.iter().enumerate() {
            self.walk(x, child_ctx(Slot::Arg(i)), scope, found);
        }
        if let Some(s) = &n.select {
            self.walk_select(s, scope, &[], found);
        }
    }

    /// The type a built-in function gives its argument `i`, for a parameter written there.
    fn argument(&self, n: &Node, i: usize, scope: &ScopeStack) -> Ctx {
        let name = n.name.as_deref().unwrap_or("");
        if !n.agg && !n.window && self.extras.function(name, n.args.len()).is_some() {
            return Ctx::None;
        }
        let text = || Ctx::Typed(Some(Fact::new("text", true)));
        let integer = || Ctx::Typed(Some(Fact::new("integer", true)));
        let real = || Ctx::Typed(Some(Fact::new("real", true)));
        let others = || {
            let facts: Vec<Fact> = n.args.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, a)| self.type_of(a, scope)).collect();
            let refs: Vec<&Fact> = facts.iter().collect();
            let ty = shared(&refs);
            Ctx::Typed(if matches!(ty, Type::Unknown) { None } else { Some(Fact { ty, non_null: true }) })
        };
        match name {
            "like" | "glob" | "regexp" | "match" | "upper" | "lower" | "trim" | "ltrim" | "rtrim" | "length" | "hex" | "unicode" | "quote" | "instr" | "replace"
            | "printf" | "format" | "concat" | "concat_ws" | "date" | "time" | "datetime" | "strftime" | "julianday" | "unixepoch" | "timediff" | "json"
            | "json_extract" | "json_type" | "json_valid" | "json_array_length" | "json_quote" | "json_pretty" | "json_insert" | "json_replace" | "json_set"
            | "json_remove" | "json_patch" | "->" | "->>" => text(),
            "substr" | "substring" => {
                if i == 0 { text() } else { integer() }
            }
            "zeroblob" | "randomblob" | "char" => integer(),
            "abs" | "round" | "acos" | "acosh" | "asin" | "asinh" | "atan" | "atan2" | "atanh" | "ceil" | "ceiling" | "cos" | "cosh" | "degrees" | "exp"
            | "floor" | "ln" | "log" | "log10" | "log2" | "mod" | "pow" | "power" | "radians" | "sin" | "sinh" | "sqrt" | "tan" | "tanh" | "trunc" => real(),
            "coalesce" | "ifnull" | "nullif" | "min" | "max" => others(),
            "iif" | "if" => {
                if i == 0 { Ctx::Bool } else { others() }
            }
            "sum" | "avg" | "total" => real(),
            _ => Ctx::None,
        }
    }

    // ---- the row count -----------------------------------------------------------------------

    fn cardinality(&self) -> (String, Option<String>) {
        let t = self.tree;
        if let Some(dml) = &t.dml {
            if t.returning.is_none() {
                return ("none".into(), None);
            }
            return match dml.kind.as_str() {
                "insert" => {
                    if dml.rows == 1 {
                        use crate::cardinality::{TRIGGER_ROW_INSERT, TRIGGER_ROW_UPDATE};
                        let triggered = |events: i32| self.skipping.get(&dml.table.to_ascii_lowercase()).is_some_and(|mask| mask & events != 0);
                        let can_skip = dml.on_error == "ignore"
                            || dml.upsert.as_ref().is_some_and(|u| !u.do_update || u.where_.is_some() || triggered(TRIGGER_ROW_UPDATE))
                            || triggered(TRIGGER_ROW_INSERT);
                        if can_skip { ("maybeOne".into(), Some("ON CONFLICT DO NOTHING, OR IGNORE, a DO UPDATE with a WHERE, or a BEFORE trigger's RAISE(IGNORE) can insert no row".into())) } else { ("one".into(), None) }
                    } else if dml.rows > 1 {
                        ("many".into(), Some(format!("a VALUES list of {} rows returns {} rows", dml.rows, dml.rows)))
                    } else {
                        ("many".into(), Some("an INSERT ... SELECT returns a row per row it reads".into()))
                    }
                }
                _ => {
                    // UPDATE or DELETE: the WHERE pins a unique key of the target, or any number.
                    match self.pinned_by_name(&dml.table, dml.where_.as_ref()) {
                        Ok(()) => ("maybeOne".into(), None),
                        Err(why) => ("many".into(), Some(why)),
                    }
                }
            };
        }
        let Some(sel) = &t.statement else { return ("none".into(), None) };
        match self.exactly_one(sel) {
            Ok(()) => ("one".into(), None),
            Err(one_why) => match self.at_most_one(sel) {
                Ok(()) => ("maybeOne".into(), Some(one_why)),
                Err(why) => ("many".into(), Some(why)),
            },
        }
    }

    fn exactly_one(&self, sel: &Sel) -> Result<(), String> {
        if sel.prior.is_some() {
            return Err("a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows".into());
        }
        if sel.values {
            return Ok(());
        }
        if !sel.group_by.is_empty() {
            return Err("GROUP BY returns one row per group, and none when nothing matches".into());
        }
        if sel.having.is_some() {
            return Err("HAVING can remove the row".into());
        }
        if sel.limit.is_some() || sel.offset.is_some() {
            return Err("a LIMIT or an OFFSET can remove the row (LIMIT 0, OFFSET 1)".into());
        }
        if sel.columns.iter().any(|c| has_aggregate(&c.expr)) {
            return Ok(());
        }
        if sel.from.is_empty() && sel.where_.is_none() {
            return Ok(());
        }
        Err("a FROM returns a row per row it matches, and only an aggregate with no GROUP BY collapses them to one".into())
    }

    fn at_most_one(&self, sel: &Sel) -> Result<(), String> {
        if let Some(limit) = &sel.limit {
            if limit.op == "integer" && limit.value.as_deref().and_then(|v| v.parse::<i64>().ok()).is_some_and(|n| n <= 1) {
                return Ok(());
            }
            return Err("a LIMIT of more than one row, or one the build cannot read, bounds nothing".into());
        }
        // An INTERSECT returns only rows both sides return, and an EXCEPT only rows of its left
        // side (`prior`, the arms before this one): at most one row when a side it draws every
        // row from is.
        if let Some(prior) = &sel.prior {
            let drawn = match sel.op.as_deref() {
                Some("intersect") => self.at_most_one(prior).is_ok() || self.arm_at_most_one(sel).is_ok(),
                Some("except") => self.at_most_one(prior).is_ok(),
                _ => false,
            };
            return if drawn { Ok(()) } else { Err("a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows".into()) };
        }
        self.arm_at_most_one(sel)
    }

    /// The proof for one SELECT arm, its set operation aside: the closure of [`Self::determined`],
    /// as `cardinality` proves it over libpg_query's tree. Every FROM item determined is one row
    /// at most; one group at most when every GROUP BY expression is pinned; one row at most from
    /// a DISTINCT over pinned values.
    fn arm_at_most_one(&self, sel: &Sel) -> Result<(), String> {
        if sel.values {
            return Err("a VALUES list returns a row per row it lists".into());
        }
        if !sel.group_by.is_empty() {
            if sel.from.iter().all(|f| !f.right) && self.determined(sel, false).iter().all(|d| *d) {
                return Ok(());
            }
            let constant = self.determined(sel, true);
            if sel.group_by.iter().all(|g| self.pinned(sel, g, &constant)) {
                return Ok(());
            }
            return Err("GROUP BY returns one row per group; the engine proves one group only when every GROUP BY expression is pinned to one value: a column of a relation whose unique key the WHERE pins, or a column the WHERE equates to a value".into());
        }
        if sel.having.is_some() {
            return Err("HAVING leaves a row per group".into());
        }
        if sel.from.is_empty() {
            return Err("a SELECT with no FROM and a WHERE can return no row or one, and the engine proves that only through a LIMIT".into());
        }
        // A RIGHT or FULL JOIN keeps the rows of each side that match nothing: two rows where
        // each side has one.
        if sel.from.iter().any(|f| f.right) {
            return Err("a RIGHT or FULL JOIN returns a row for each side that matches nothing".into());
        }
        let determined = self.determined(sel, false);
        if determined.iter().all(|d| *d) {
            return Ok(());
        }
        if sel.distinct {
            let constant = self.determined(sel, true);
            if !sel.columns.is_empty() && sel.columns.iter().all(|c| self.pinned(sel, &c.expr, &constant)) {
                return Ok(());
            }
        }
        let i = determined.iter().position(|d| !d).unwrap_or(0);
        let f = &sel.from[i];
        let Some(table) = f.table.as_deref().filter(|_| f.subquery.is_none() && !f.tab_func) else {
            return Err(format!("`{}` in the FROM is a subquery or a table-valued function that returns any number of rows", f.alias.as_deref().or(f.table.as_deref()).unwrap_or("?")));
        };
        let keys = self.tree.tables.get(table).map(|t| t.unique_keys.clone()).unwrap_or_default();
        if keys.is_empty() {
            return Err(format!("`{table}` has no unique index the engine can read, so nothing bounds the rows a WHERE matches"));
        }
        let shown: Vec<String> = keys.iter().map(|k| format!("({})", k.join(", "))).collect();
        if sel.from.len() == 1 {
            Err(format!("the WHERE does not equate every column of a unique key of `{table}` — it has {}", shown.join(", ")))
        } else {
            let name = f.alias.as_deref().unwrap_or(table);
            Err(format!("a join returns a row per pair it matches, and the WHERE and ON clauses do not equate every column of a unique key of `{name}` to values or to columns of relations pinned to one row — it has {}", shown.join(", ")))
        }
    }

    /// Which FROM items are determined: every row carries the same row of each, or its null
    /// extension (see `cardinality`'s `From`). A table is, when a unique key's every column is
    /// equated to a value or to a column of a determined item; a subquery or a WITH query, when the
    /// proof bounds it. SQLite moves every ON term into the WHERE: a term from an outer join's ON
    /// pins only that join's right side, and with `constant`, which asks that every row carry the
    /// same row of an item, never: some rows may meet the null extension instead.
    fn determined(&self, sel: &Sel, constant: bool) -> Vec<bool> {
        let n = sel.from.len();
        let index: HashMap<i64, usize> = sel.from.iter().enumerate().map(|(i, f)| (f.cursor, i)).collect();
        let terms = conjuncts(sel.where_.as_ref());
        let mut done = vec![false; n];
        loop {
            let mut changed = false;
            for i in 0..n {
                if done[i] {
                    continue;
                }
                let f = &sel.from[i];
                let pinned = if let Some(sub) = &f.subquery {
                    !(constant && f.outer) && self.at_most_one(sub).is_ok()
                } else if let (Some(table), false, false) = (f.table.as_deref(), f.tab_func, f.virtual_) {
                    // A WHERE term's `=` rejects the null extension, so it pins even an outer
                    // join's right side for every row.
                    let usable: Vec<&Node> = terms.iter().copied().filter(|t| !t.outer_on || (!constant && t.join == f.cursor)).collect();
                    self.key_pinned(table, f.cursor, &usable, &done, &index)
                } else {
                    false
                };
                if pinned {
                    done[i] = true;
                    changed = true;
                }
            }
            if !changed {
                return done;
            }
        }
    }

    /// Whether some unique key of `table` (read through `cursor`) has every column equated by
    /// one of `terms` to a value: one that reads no column of this FROM's undetermined items (a
    /// column of a determined item is one value, and so is an enclosing query's), calls nothing
    /// that changes from row to row, and carries no COLLATE.
    fn key_pinned(&self, table: &str, cursor: i64, terms: &[&Node], done: &[bool], index: &HashMap<i64, usize>) -> bool {
        let Some(info) = self.tree.tables.get(table) else { return false };
        let pins: Vec<&str> = terms.iter().filter_map(|t| self.pins(t, cursor, done, index)).collect();
        info.unique_keys.iter().any(|key| !key.is_empty() && key.iter().all(|c| pins.iter().any(|p| p.eq_ignore_ascii_case(c))))
    }

    /// The column of `cursor` a term equates to a value, if it does, under a comparison that
    /// leaves the column as it is ([`keeps_column`]) and a collation its unique index holds to:
    /// the left operand's column's, else the right's (`sqlite3BinaryCompareCollSeq`), which must
    /// be BINARY or the key's own.
    fn pins<'n>(&self, t: &'n Node, cursor: i64, done: &[bool], index: &HashMap<i64, usize>) -> Option<&'n str> {
        if t.op != "=" {
            return None;
        }
        let (l, r) = (t.left.as_deref()?, t.right.as_deref()?);
        let collation = column_of(l).or_else(|| column_of(r)).and_then(|c| c.coll.as_deref()).unwrap_or("BINARY");
        [(l, r), (r, l)].into_iter().find_map(|(col, other)| {
            let value = col.op == "column"
                && col.cursor == cursor
                && !has_op(other, "collate")
                && keeps_column(col, other)
                && (collation.eq_ignore_ascii_case("BINARY") || col.coll.as_deref().is_some_and(|c| c.eq_ignore_ascii_case(collation)))
                && !self.calls_volatile(other)
                && other.select.is_none()
                && cursors(other).is_some_and(|read| read.iter().all(|c| *c != cursor && index.get(c).is_none_or(|i| done[*i])));
            value.then_some(col.column.as_deref()).flatten()
        })
    }

    /// Whether every row carries the same value of `e`: a column of a constant item, a column the
    /// WHERE (not an outer join's ON) equates to a value, or a value itself.
    fn pinned(&self, sel: &Sel, e: &Node, constant: &[bool]) -> bool {
        let index: HashMap<i64, usize> = sel.from.iter().enumerate().map(|(i, f)| (f.cursor, i)).collect();
        if e.op == "column" {
            if index.get(&e.cursor).is_some_and(|i| constant[*i]) {
                return true;
            }
            let terms = conjuncts(sel.where_.as_ref());
            return terms.iter().filter(|t| !t.outer_on).filter_map(|t| self.pins(t, e.cursor, constant, &index)).any(|c| e.column.as_deref().is_some_and(|name| name.eq_ignore_ascii_case(c)));
        }
        e.select.is_none() && !self.calls_volatile(e) && cursors(e).is_some_and(|read| read.iter().all(|c| !index.contains_key(c)))
    }

    /// The DML form: the target table's columns have no stable cursor, so they match by name.
    fn pinned_by_name(&self, table: &str, where_: Option<&Node>) -> Result<(), String> {
        let equated = self.equated(where_);
        let Some(info) = self.tree.tables.get(table) else { return Err(format!("`{table}` is not a table the engine can read")) };
        let names: HashSet<String> = equated.iter().map(|(_, n)| n.to_ascii_lowercase()).collect();
        if info.unique_keys.iter().any(|key| !key.is_empty() && key.iter().all(|c| names.contains(&c.to_ascii_lowercase()))) {
            return Ok(());
        }
        if info.unique_keys.is_empty() {
            return Err(format!("`{table}` has no unique index the engine can read, so nothing bounds the rows a WHERE matches"));
        }
        let shown: Vec<String> = info.unique_keys.iter().map(|k| format!("({})", k.join(", "))).collect();
        Err(format!("the WHERE does not equate every column of a unique key of `{table}` — it has {}", shown.join(", ")))
    }

    /// The columns the WHERE equates to a value: `col = <no column anywhere>` under ANDs, where
    /// the value calls nothing that changes from row to row.
    fn equated(&self, where_: Option<&Node>) -> HashSet<(i64, String)> {
        let mut out = HashSet::new();
        for n in conjuncts(where_) {
            if n.op != "=" || n.outer_on {
                continue;
            }
            if let (Some(l), Some(r)) = (n.left.as_deref(), n.right.as_deref()) {
                for (col, other) in [(l, r), (r, l)] {
                    // An explicit COLLATE (NOCASE) may compare more loosely than the
                    // unique index distinguishes.
                    if col.op == "column" && !reads_column(other) && !has_op(other, "collate") && keeps_column(col, other) && !self.calls_volatile(other) {
                        if let Some(name) = &col.column {
                            out.insert((col.cursor, name.clone()));
                        }
                    }
                }
            }
        }
        out
    }

    /// Whether `n` calls a function that may give each row another value: `random()`,
    /// `changes()`, a date or time function (`'now'` moves), or one the host registers or the
    /// engine does not know, whose determinism it cannot tell.
    fn calls_volatile(&self, n: &Node) -> bool {
        const VOLATILE: &[&str] = &["random", "randomblob", "changes", "total_changes", "last_insert_rowid", "date", "time", "datetime", "julianday", "unixepoch", "strftime", "timediff"];
        if n.op == "function" {
            let name = n.name.as_deref().unwrap_or("").to_ascii_lowercase();
            if VOLATILE.contains(&name.as_str()) || self.extras.function(&name, n.args.len()).is_some() || !KNOWN_FUNCTIONS.contains(&name.as_str()) {
                return true;
            }
        }
        n.left.as_deref().is_some_and(|x| self.calls_volatile(x)) || n.right.as_deref().is_some_and(|x| self.calls_volatile(x)) || n.list.iter().any(|x| self.calls_volatile(x)) || n.args.iter().any(|x| self.calls_volatile(x))
    }
}

/// Whether comparing key column `col` with `other` leaves the column's values as they are. SQLite
/// applies NUMERIC affinity to both sides where either side's affinity is INTEGER, REAL or NUMERIC,
/// and one side's affinity to both where the other has none, so a TEXT key compared with a number
/// reads '01' and '1' as 1, and a key of no affinity compared with a TEXT side reads 7 as '7'. A
/// numeric key converts nothing it holds: what stays text there is no number.
fn keeps_column(col: &Node, other: &Node) -> bool {
    let numeric = |a: &str| matches!(a, "integer" | "real" | "numeric");
    let none = |a: &str| matches!(a, "blob" | "none");
    numeric(&col.aff) || none(&other.aff) || (col.aff == "text" && other.aff == "text")
}

/// The column an operand reads through casts and unary plus, as SQLite looks for the collation.
fn column_of(n: &Node) -> Option<&Node> {
    match n.op.as_str() {
        "column" => Some(n),
        "cast" | "pos" => n.left.as_deref().and_then(column_of),
        _ => None,
    }
}

/// SQLite's built-in scalar functions the engine's table reads, by name.
const KNOWN_FUNCTIONS: &[&str] = &[
    "coalesce", "ifnull", "nullif", "iif", "if", "min", "max", "abs", "round", "length", "instr", "unicode", "json_array_length", "json_valid", "octet_length", "sign",
    "like", "glob", "regexp", "match", "likely", "unlikely", "likelihood", "upper", "lower", "trim", "ltrim", "rtrim", "substr", "substring", "replace", "char",
    "printf", "format", "concat", "concat_ws", "json", "jsonb", "json_array", "json_object", "json_insert", "json_replace", "json_set", "json_remove", "json_patch",
    "json_quote", "json_pretty", "json_type", "hex", "quote", "typeof", "zeroblob", "json_extract", "jsonb_extract", "->", "->>", "acos", "acosh", "asin", "asinh",
    "atan", "atan2", "atanh", "ceil", "ceiling", "cos", "cosh", "degrees", "exp", "floor", "ln", "log", "log10", "log2", "mod", "pow", "power", "radians", "sin",
    "sinh", "sqrt", "tan", "tanh", "trunc", "pi",
];

/// The terms of a condition's top-level `AND`.
fn conjuncts(n: Option<&Node>) -> Vec<&Node> {
    let mut out = Vec::new();
    let mut stack: Vec<&Node> = n.into_iter().collect();
    while let Some(n) = stack.pop() {
        if n.op == "and" {
            stack.extend(n.left.as_deref());
            stack.extend(n.right.as_deref());
        } else {
            out.push(n);
        }
    }
    out
}

/// The cursors an expression reads, or `None` when it holds a subquery, whose reads the walk
/// does not place.
fn cursors(n: &Node) -> Option<Vec<i64>> {
    if n.select.is_some() {
        return None;
    }
    let mut out = Vec::new();
    if n.op == "column" {
        out.push(n.cursor);
    }
    for child in n.left.iter().chain(n.right.iter()).map(|b| b.as_ref()).chain(n.list.iter()).chain(n.args.iter()) {
        out.extend(cursors(child)?);
    }
    Some(out)
}

/// The type of `a op b` for `+`, `-`, `*`, `/`, `%`: integer stays integer, anything else is real.
fn arithmetic(l: &Type, r: &Type) -> Type {
    match (scalar_name(l), scalar_name(r)) {
        (Some("integer"), Some("integer")) => Type::Scalar { name: "integer".into() },
        _ => Type::Scalar { name: "real".into() },
    }
}

/// Whether a node of this operator sits anywhere in `n`.
fn has_op(n: &Node, op: &str) -> bool {
    n.op == op || n.left.as_deref().is_some_and(|x| has_op(x, op)) || n.right.as_deref().is_some_and(|x| has_op(x, op)) || n.list.iter().any(|x| has_op(x, op)) || n.args.iter().any(|x| has_op(x, op))
}

fn has_aggregate(n: &Node) -> bool {
    if n.op == "function" && n.agg && !n.window {
        return true;
    }
    n.left.as_deref().is_some_and(has_aggregate) || n.right.as_deref().is_some_and(has_aggregate) || n.list.iter().any(has_aggregate) || n.args.iter().any(has_aggregate)
}

/// Whether an expression reads any column, at any depth (a subquery's columns included).
fn reads_column(n: &Node) -> bool {
    if n.op == "column" {
        return true;
    }
    n.left.as_deref().is_some_and(reads_column) || n.right.as_deref().is_some_and(reads_column) || n.list.iter().any(reads_column) || n.args.iter().any(reads_column) || n.select.is_some()
}

/// The type the context an expression stands in gives its operand, when it gives one.
fn outer_type(ctx: &Ctx) -> Ctx {
    match ctx {
        Ctx::Typed(Some(f)) => Ctx::Typed(Some(f.clone())),
        _ => Ctx::None,
    }
}

/// The argument position of a slot (0 for any other slot).
fn i_of(slot: Slot) -> usize {
    match slot {
        Slot::Arg(i) => i,
        _ => 0,
    }
}

/// SQLite's five affinity rules over a declared type.
pub fn affinity(declared: &str) -> &'static str {
    let d = declared.to_ascii_uppercase();
    if d.contains("INT") {
        "integer"
    } else if d.contains("CHAR") || d.contains("CLOB") || d.contains("TEXT") {
        "text"
    } else if d.contains("BLOB") || d.is_empty() {
        "blob"
    } else if d.contains("REAL") || d.contains("FLOA") || d.contains("DOUB") {
        "real"
    } else {
        "numeric"
    }
}

/// A parameter's context while walking.
#[derive(Clone)]
enum Ctx {
    Typed(Option<Fact>),
    Bool,
    None,
}

#[derive(Clone, Copy)]
enum Slot {
    Left,
    Right,
    List(usize),
    Arg(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ddl() -> Vec<String> {
        vec![
            "create table users (id integer primary key, name text not null, email text unique, score real, avatar blob, created_at text not null default current_timestamp)".to_string(),
            "create table posts (id integer primary key, user_id integer not null references users(id), title text not null, body text)".to_string(),
        ]
    }

    fn analysis(sql: &str) -> Analysis {
        analyze_ddl(sql, &ddl(), &Extras::default()).analysis
    }

    fn col(a: &Analysis, name: &str) -> (String, bool) {
        let c = a.columns.iter().find(|c| c.name == name).unwrap_or_else(|| panic!("no column {name}"));
        (scalar_name(&c.ty).unwrap_or("unknown").to_string(), c.nullable)
    }

    fn param(a: &Analysis, name: &str) -> String {
        let p = a.params.iter().find(|p| p.name == name).unwrap_or_else(|| panic!("no param {name}"));
        scalar_name(&p.ty).unwrap_or("unknown").to_string()
    }

    #[test]
    fn the_postgres_nullability_rules_hold_on_sqlite() {
        let nullable_param = |sql: &str, name: &str| analysis(sql).params.iter().find(|p| p.name == name).is_some_and(|p| p.nullable);
        // A parameter the statement tests for NULL or coalesces takes NULL; any other is a value.
        assert!(nullable_param("select id from users where (:p is null or score > :p)", "p"));
        assert!(nullable_param("select coalesce(:s, 0) as x", "s"));
        assert!(!nullable_param("select id from users where score > :p", "p"));
        assert_eq!(col(&analysis("select upper(:s) as u"), "u"), ("text".into(), false));
        // A strict WHERE narrows the column it tests, and reduces the outer join of its item.
        assert_eq!(col(&analysis("select email from users where email is not null"), "email"), ("text".into(), false));
        assert_eq!(col(&analysis("select email from users where lower(email) = :e or email like 'a%'"), "email").1, false);
        assert_eq!(col(&analysis("select email from users where email like 'a%' or id = 1"), "email").1, true);
        assert_eq!(col(&analysis("select email from users where lower(email) = :e and email like 'a%'"), "email").1, false);
        let reduced = analysis("select p.title, u.name from posts p left join users u on u.id = p.user_id where u.email = :e");
        assert_eq!(col(&reduced, "name"), ("text".into(), false));
        assert_eq!(col(&analysis("select p.title, u.name from posts p left join users u on u.id = p.user_id"), "name"), ("text".into(), true));
        // An ON term is not a WHERE test: the posts that meet no user still have NULL there.
        assert_eq!(col(&analysis("select p.title, u.name from posts p left join users u on u.id = p.user_id and u.email = :e"), "name"), ("text".into(), true));
        // Under a plain GROUP BY no group is empty.
        let grouped = analysis("select user_id, max(title) as t, min(id) as m, group_concat(title) as g from posts group by user_id");
        assert_eq!((col(&grouped, "t").1, col(&grouped, "m").1, col(&grouped, "g").1), (false, false, false));
        assert!(col(&analysis("select max(title) as t from posts"), "t").1);
        // `concat` skips NULLs; `concat_ws` is NULL only for a NULL separator.
        let concat = analysis("select concat(email, 'x') as c, concat_ws(',', email) as w, concat_ws(email, 'a') as n from users");
        assert_eq!((col(&concat, "c").1, col(&concat, "w").1, col(&concat, "n").1), (false, false, true));
    }

    #[test]
    fn a_parameter_takes_the_type_its_call_case_or_compound_stands_in() {
        assert_eq!(param(&analysis("select id from users where name = coalesce(:a, :b)"), "a"), "text");
        assert_eq!(param(&analysis("select id from users where name = coalesce(:a, :b)"), "b"), "text");
        assert_eq!(param(&analysis("select id from users where name = case when id > 1 then :a else :b end"), "a"), "text");
        assert_eq!(param(&analysis("select id from users where name = iif(id > 1, :a, 'x')"), "a"), "text");
        assert_eq!(param(&analysis("select case :kind when 'a' then 1 else 2 end as k from users"), "kind"), "text");
        assert_eq!(param(&analysis("select name from users union all select :x"), "x"), "text");
        assert_eq!(param(&analysis("select :x union all select name from users"), "x"), "text");
        // A call whose result is not an argument's gives none of its own context.
        assert_eq!(param(&analysis("select id from users where name = printf(:f, 1)"), "f"), "text");
        assert_eq!(param(&analysis("select :x as v"), "x"), "unknown");
    }

    #[test]
    fn joins_grouping_distinct_and_set_operations_prove_rows_as_on_postgres() {
        let rows = |sql: &str| analysis(sql).cardinality;
        for sql in [
            "select p.title, u.name from posts p join users u on u.id = p.user_id where p.id = :id",
            "select p.title, u.name from posts p left join users u on u.id = p.user_id where p.id = :id",
            "select p.title, u.name from posts p, users u where u.id = p.user_id and p.id = :id",
            "select a.name from users a join users b on b.id = a.id where a.id = :id",
            "select s.name from (select * from users where id = :id) s",
            "select user_id, count(*) as n from posts where user_id = :u group by user_id",
            "select distinct user_id from posts where user_id = :u",
            "select id from users where id = :id intersect select user_id from posts",
            "select id from users where id = :id except select user_id from posts",
        ] {
            assert_eq!(rows(sql), "maybeOne", "{sql}");
        }
        for sql in [
            // Many posts per user.
            "select p.title from posts p join users u on u.id = p.user_id where u.id = :id",
            "select title, count(*) as n from posts where user_id = :u group by title",
            "select user_id from posts except select id from users where id = :id",
            // A FULL JOIN keeps each side's unmatched rows.
            "select p.title, u.name from posts p full join users u on u.id = p.user_id where p.id = :id",
            // A volatile value is evaluated for each row.
            "select name from users where id = abs(random()) % 3",
            "delete from users where id = abs(random()) % 3 returning id",
        ] {
            assert_eq!(rows(sql), "many", "{sql}");
        }
    }

    #[test]
    fn an_outer_joins_on_term_pins_its_right_side_but_groups_nothing() {
        // SQLite moves the ON terms into the WHERE: `u.id = :id` restricts which user a post
        // meets, and the posts that meet none have NULL there, a second group.
        assert_eq!(analysis("select u.name, count(*) as n from posts p left join users u on u.id = :id and u.id = p.user_id group by u.name").cardinality, "many");
        assert_eq!(analysis("select u.name, count(*) as n from posts p left join users u on u.id = p.user_id where u.id = :id group by u.name").cardinality, "maybeOne");
        // The ON term still pins the right side against a pinned left one.
        assert_eq!(analysis("select p.title, u.name from posts p left join users u on u.id = :u where p.id = :id").cardinality, "maybeOne");
    }

    #[test]
    fn an_insert_returns_no_row_where_something_can_skip_it() {
        let count = |sql: &str, extra: &[&str]| {
            let mut schema = ddl();
            schema.extend(extra.iter().map(|s| s.to_string()));
            analyze_ddl(sql, &schema, &Extras::default()).analysis.cardinality
        };
        let insert = "insert into users (name) values (:n) returning id";
        assert_eq!(count(insert, &[]), "one");
        // A BEFORE trigger's RAISE(IGNORE) skips the row; an AFTER trigger cannot.
        assert_eq!(count(insert, &["create trigger skip before insert on users begin select raise(ignore) where new.name = 'x'; end"]), "maybeOne");
        assert_eq!(count(insert, &["create trigger untimed insert on users begin select 1; end"]), "maybeOne", "BEFORE is the default");
        assert_eq!(count(insert, &["create trigger later after insert on users begin select 1; end"]), "one");
        // `DO UPDATE … WHERE` leaves a conflicting row alone and returns nothing for it.
        assert_eq!(count("insert into users (id, name) values (:id, :n) on conflict (id) do update set name = excluded.name where users.score > 1 returning id", &[]), "maybeOne");
        assert_eq!(count("insert into users (id, name) values (:id, :n) on conflict (id) do update set name = excluded.name returning id", &[]), "one");
    }

    #[test]
    fn a_unique_key_bounds_only_a_comparison_under_its_own_collation() {
        let count = |sql: &str| {
            let mut schema = ddl();
            schema.push("create table c (id integer primary key, e text collate nocase, b text unique)".into());
            schema.push("create unique index c_e on c (e collate binary)".into());
            analyze_ddl(sql, &schema, &Extras::default()).analysis.cardinality
        };
        // `e` compares under NOCASE, and its unique index tells 'A' from 'a'.
        assert_eq!(count("select id from c where e = :e"), "many");
        assert_eq!(count("select id from c where b = :b"), "maybeOne");
        assert_eq!(count("select id from c where b = :b collate nocase"), "many");
        assert_eq!(count("select e from c where id = :id"), "maybeOne");
    }

    #[test]
    fn a_key_holds_only_where_the_comparison_leaves_the_column_as_it_is() {
        let count = |sql: &str| {
            let mut schema = ddl();
            schema.push("create table t (id integer primary key, code text unique)".into());
            schema.push("create table b (id integer primary key, n integer, nc text collate nocase)".into());
            schema.push("create table a (id integer primary key, k unique, kt text unique)".into());
            schema.push("create table rt (rowid integer, v text)".into());
            analyze_ddl(sql, &schema, &Extras::default()).analysis.cardinality
        };
        // A side of INTEGER affinity applies NUMERIC affinity to a TEXT or untyped key: '01' and
        // '1' compare equal to 1, and so do 7 and '7'.
        assert_eq!(count("select id from t where code = cast(:c as integer)"), "many");
        assert_eq!(count("select a.id from b join a on a.k = b.n where b.id = 1"), "many");
        assert_eq!(count("update t set id = id where code = cast(:c as integer) returning id"), "many");
        assert_eq!(count("select id from t where code = :c"), "maybeOne");
        assert_eq!(count("select a.id from b join a on a.kt = b.nc where b.id = 1"), "maybeOne");
        // A column on the left decides the collation: NOCASE meets 'x' and 'X'.
        assert_eq!(count("select a.id from b join a on b.nc = a.kt where b.id = 1"), "many");
        // A declared column named rowid hides the rowid; `_rowid_` still reads it.
        assert_eq!(count("select v from rt where rowid = :r"), "many");
        assert_eq!(count("select v from rt where _rowid_ = :r"), "maybeOne");
    }

    #[test]
    fn a_function_the_host_registers_over_a_built_in_proves_nothing_strict() {
        let lower = Extras { total: vec![TotalFunction { name: "lower".into(), arity: 1, returns: oid::TEXT }] };
        let nullable = |extras: &Extras| col(&analyze_ddl("select email from users where lower(email) = :e", &ddl(), extras).analysis, "email").1;
        assert!(!nullable(&Extras::default()));
        assert!(nullable(&lower));
    }

    #[test]
    fn unary_plus_is_its_operand() {
        // `+name` returns the text itself, and SQLite writes a one-element `x IN (e)` as `x = +e`.
        let a = analysis("select +name as p, -score as m from users where name in (:n) and id in (:i)");
        assert_eq!(col(&a, "p"), ("text".into(), false));
        assert_eq!(col(&a, "m"), ("real".into(), true));
        assert_eq!(param(&a, "n"), "text");
        assert_eq!(param(&a, "i"), "integer");
    }

    #[test]
    fn columns_from_provenance_and_rules() {
        let a = analysis("select id, name, email, score, length(email) as l, coalesce(email, 'x') as e, id * 2 as d, id / 2 as h, count(*) as n from users");
        assert_eq!(col(&a, "id"), ("integer".into(), false));
        assert_eq!(col(&a, "name"), ("text".into(), false));
        assert_eq!(col(&a, "email"), ("text".into(), true));
        assert_eq!(col(&a, "score"), ("real".into(), true));
        assert_eq!(col(&a, "l"), ("integer".into(), true));
        assert_eq!(col(&a, "e"), ("text".into(), false));
        assert_eq!(col(&a, "d"), ("integer".into(), false));
        assert_eq!(col(&a, "h"), ("integer".into(), false));
        assert_eq!(col(&a, "n"), ("integer".into(), false));
        assert_eq!(a.cardinality, "one");
    }

    #[test]
    fn outer_join_and_subquery_provenance() {
        let a = analysis("select u.id, p.title from users u left join posts p on p.user_id = u.id");
        assert_eq!(col(&a, "title"), ("text".into(), true));
        let a = analysis("select t.id, t.name from (select id, name from users) t where t.id = :id");
        assert_eq!(col(&a, "id"), ("integer".into(), false));
        assert_eq!(col(&a, "name"), ("text".into(), false));
        assert_eq!(param(&a, "id"), "integer");
    }

    #[test]
    fn parameters_from_context() {
        let a = analysis("select id from users where email = :email and score >= :min and id in (:a, :b) and created_at between :from and :to and name like :q and lower(name) = :n limit :lim offset :off");
        assert_eq!(param(&a, "email"), "text");
        assert_eq!(param(&a, "min"), "real");
        assert_eq!(param(&a, "a"), "integer");
        assert_eq!(param(&a, "from"), "text");
        assert_eq!(param(&a, "q"), "text");
        assert_eq!(param(&a, "n"), "text");
        assert_eq!(param(&a, "lim"), "integer");
        assert_eq!(param(&a, "off"), "integer");
        let a = analysis("insert into users (email, name, score) values (:e, upper(:n), :s * 2) returning id");
        assert_eq!(param(&a, "e"), "text");
        assert_eq!(param(&a, "n"), "text");
        assert_eq!(param(&a, "s"), "integer");
        assert_eq!(a.cardinality, "one");
        let a = analysis("update users set name = :name, score = :score where id = :id returning id");
        assert_eq!(param(&a, "name"), "text");
        assert_eq!(param(&a, "score"), "real");
        assert_eq!(param(&a, "id"), "integer");
        assert_eq!(a.cardinality, "maybeOne");
    }

    #[test]
    fn cardinality_by_keys() {
        assert_eq!(analysis("select id from users where email = :email").cardinality, "maybeOne");
        assert_eq!(analysis("select id from users where name = :name").cardinality, "many");
        assert_eq!(analysis("select u.id, count(p.id) as n from users u left join posts p on p.user_id = u.id where u.id = :id group by u.id").cardinality, "maybeOne");
        assert_eq!(analysis("select id from users limit 1").cardinality, "maybeOne");
        assert_eq!(analysis("insert or ignore into users (email, name) values (:e, :n) returning id").cardinality, "maybeOne");
        assert_eq!(analysis("insert into users (email, name) values (:e, :n)").cardinality, "none");
        assert_eq!(analysis("delete from users where id = :id returning id").cardinality, "maybeOne");
        assert_eq!(analysis("select id from users union all select user_id from posts").cardinality, "many");
    }

    /// The whole path, schema query included, as the host drives it.
    fn analyzed(sql: &str) -> Analysis {
        let ddl = [
            "create table users (id integer primary key, name text not null, email text unique, score real, avatar blob, created_at text not null default current_timestamp)",
        ];
        let rows = ddl.iter().map(|d| vec![serde_json::Value::String(d.to_string())]).collect();
        let answers = std::collections::HashMap::from([(format!("query:{DDL_QUERY}|[]"), crate::api::Answer::Rows { rows })]);
        let host = Host::new(&answers);
        analyze(sql, &lex::lex(sql, Dialect::Sqlite).unwrap(), &host).expect("the schema was answered")
    }

    #[test]
    fn prepares_the_numbered_rewrite() {
        // SQLite's own `:auth` would end at the dot; the rewrite keeps the whole name one parameter.
        let a = analyzed("select name from users where email = :auth.email and id = :auth.id and score > :auth.email");
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!(param(&a, "auth.email"), "text");
        assert_eq!(param(&a, "auth.id"), "integer");
        assert_eq!(a.rewritten.sql, "select name from users where email = ? and id = ? and score > ?");
        assert_eq!(a.rewritten.order, ["auth.email", "auth.id", "auth.email"]);
        // Positional forms: SQLite's `?3` is the parameter `3`, and a bare `?` the next number.
        assert_eq!(param(&analyzed("select name from users where id = ?3"), "3"), "integer");
        assert_eq!(param(&analyzed("select name from users where email = ?"), "1"), "text");
        // An error SQLite reports in the rewrite lands on the author's text.
        let sql = "select :auth.user_id, nope from users";
        let a = analyzed(sql);
        assert_eq!(a.diagnostics.len(), 1, "{:?}", a.diagnostics);
        assert_eq!((a.diagnostics[0].start, a.diagnostics[0].length), (sql.find("nope").unwrap(), 4));
    }

    #[test]
    fn a_column_declared_without_a_type_holds_any_value() {
        let ddl = vec!["CREATE TABLE t (a, b TEXT, c ANY)".to_string(), "CREATE TABLE s (x ANY, y INTEGER) STRICT".to_string()];
        let r = analyze_ddl("SELECT t.a, t.b, s.x, s.y FROM t, s WHERE t.a = :a AND s.x = :x", &ddl, &Extras::default());
        let a = &r.analysis;
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        // `t.a = :a` and `s.x = :x` keep NULL out of the rows they return.
        assert_eq!(col(a, "a"), ("unknown".into(), false));
        assert_eq!(col(a, "b"), ("text".into(), true));
        assert_eq!(col(a, "x"), ("unknown".into(), false));
        assert_eq!(col(a, "y"), ("integer".into(), true));
        assert_eq!((param(a, "a"), param(a, "x")), ("unknown".into(), "unknown".into()));
        // In a table that is not STRICT, `ANY` is a declared type with NUMERIC affinity, not a wildcard.
        let r = analyze_ddl("SELECT c FROM t", &ddl, &Extras::default());
        assert_eq!(col(&r.analysis, "c"), ("numeric".into(), true));
    }

    #[test]
    fn the_direct_entry_reads_a_schema_it_is_handed() {
        let ddl = vec![
            "CREATE TABLE todos (id uuid PRIMARY KEY NOT NULL, owner_id text NOT NULL, org_id uuid, title varchar(200) NOT NULL, done boolean NOT NULL DEFAULT 0, UNIQUE (owner_id, title))".to_string(),
            "CREATE INDEX todos_org ON todos (org_id)".to_string(),
        ];
        let sql = "SELECT t.id, t.title, t.org_id, t.done, count(*) OVER () AS n FROM todos t WHERE t.owner_id = :auth.user_id AND t.title = :title";
        let r = analyze_ddl(sql, &ddl, &Extras::default());
        let a = &r.analysis;
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!(a.kind, "select");
        assert!(r.readonly);
        assert_eq!(col(a, "id"), ("numeric".into(), false));
        assert_eq!(col(a, "title"), ("text".into(), false));
        assert_eq!(col(a, "org_id"), ("numeric".into(), true));
        assert_eq!(col(a, "n"), ("integer".into(), false));
        assert_eq!(r.declared, [Some("uuid".to_string()), Some("varchar(200)".into()), Some("uuid".into()), Some("boolean".into()), None]);
        assert_eq!(param(a, "auth.user_id"), "text");
        assert_eq!(param(a, "title"), "text");
        assert_eq!(a.cardinality, "maybeOne");
        assert_eq!(a.rewritten.sql, "SELECT t.id, t.title, t.org_id, t.done, count(*) OVER () AS n FROM todos t WHERE t.owner_id = ? AND t.title = ?");
        // A write: not read-only, and its RETURNING typed from the target.
        let r = analyze_ddl("INSERT INTO todos (id, owner_id, title) VALUES (:id, :owner, :title) RETURNING id, done", &ddl, &Extras::default());
        assert!(r.analysis.diagnostics.is_empty(), "{:?}", r.analysis.diagnostics);
        assert!(!r.readonly);
        assert_eq!((r.analysis.kind.as_str(), r.analysis.cardinality.as_str()), ("insert", "one"));
        assert_eq!(col(&r.analysis, "done"), ("numeric".into(), false));
        // The unique keys of the table it read: the primary key's index and the UNIQUE constraint's.
        assert_eq!(r.unique_keys.len(), 1, "{:?}", r.unique_keys);
        let mut keys = r.unique_keys["todos"].clone();
        keys.sort();
        assert_eq!(keys, [vec!["id".to_string()], vec!["owner_id".into(), "title".into()], vec!["rowid".into()]]);
        // A lex error comes back as the replayed path reports it.
        let r = analyze_ddl("SELECT id FROM todos WHERE id = :a AND title = ?", &ddl, &Extras::default());
        assert_eq!(r.analysis.diagnostics.len(), 1, "{:?}", r.analysis.diagnostics);
        assert!(r.declared.is_empty() && !r.readonly);
    }

    #[test]
    fn the_unique_keys_of_every_table_read() {
        let mut schema = ddl();
        schema.push("create unique index users_lower_name on users (lower(name))".into());
        schema.push("create unique index posts_title on posts (title) where body is not null".into());
        schema.push("create table tags (label text not null, post_id integer not null, primary key (post_id, label)) without rowid".into());
        schema.push("create view titled as select p.id, p.title from posts p".into());
        let r = analyze_ddl("select t.title, (select count(*) from tags g where g.post_id = t.id) as n from titled t where exists (select 1 from users u where u.id = :id)", &schema, &Extras::default());
        assert!(r.analysis.diagnostics.is_empty(), "{:?}", r.analysis.diagnostics);
        let mut tables: Vec<&String> = r.unique_keys.keys().collect();
        tables.sort();
        // The view reads through to `posts`; a view has no keys of its own.
        assert_eq!(tables, ["posts", "tags", "users"]);
        // The INTEGER PRIMARY KEY and the UNIQUE column; an expression index is no key.
        assert_eq!(r.unique_keys["users"], [vec!["id".to_string()], vec!["email".into()]]);
        // A partial index is no key either.
        assert_eq!(r.unique_keys["posts"], [vec!["id".to_string()]]);
        // WITHOUT ROWID: no rowid, only the primary key.
        assert_eq!(r.unique_keys["tags"], [vec!["post_id".to_string(), "label".into()]]);
        // A statement that does not prepare has none.
        assert!(analyze_ddl("select nope from users", &schema, &Extras::default()).unique_keys.is_empty());
    }

    fn html() -> Extras {
        Extras { total: vec![TotalFunction { name: "porgapi_html".into(), arity: 1, returns: oid::TEXT }] }
    }

    #[test]
    fn a_host_function_prepares_and_is_typed_from_its_argument() {
        let r = analyze_ddl("select porgapi_html(name) as h, porgapi_html(email) as e, upper(porgapi_html(:q)) as q from users", &ddl(), &html());
        let a = &r.analysis;
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!(col(a, "h"), ("text".into(), false));
        assert_eq!(col(a, "e"), ("text".into(), true));
        // The parameter is a non-null value, so the call over it is too.
        assert_eq!(col(a, "q"), ("text".into(), false));
        assert_eq!(r.declared, [None, None, None]);
        // The same call without the registration, and with the wrong argument count: SQLite's own
        // message, at the call.
        let sql = "select id, porgapi_html(name) as h from users";
        for (extras, message) in [(Extras::default(), "no such function: porgapi_html"), (Extras { total: vec![TotalFunction { name: "porgapi_html".into(), arity: 2, returns: oid::TEXT }] }, "wrong number of arguments to function porgapi_html()")] {
            let a = analyze_ddl(sql, &ddl(), &extras).analysis;
            assert_eq!(a.diagnostics.len(), 1, "{:?}", a.diagnostics);
            assert_eq!(a.diagnostics[0].message, message);
            assert_eq!((a.diagnostics[0].start, a.diagnostics[0].code), (sql.find("porgapi_html").unwrap(), 6));
            assert!(a.columns.is_empty());
        }
        // Any argument count, and a schema that calls the function replays.
        let any = Extras { total: vec![TotalFunction { name: "porgapi_html".into(), arity: -1, returns: oid::TEXT }] };
        let mut schema = ddl();
        schema.push("create view safe_users as select id, porgapi_html(name) as name from users".into());
        let a = analyze_ddl("select name, porgapi_html(name, name) as twice from safe_users", &schema, &any).analysis;
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!(col(&a, "name"), ("text".into(), false));
        assert_eq!(col(&a, "twice"), ("text".into(), false));
    }

    #[test]
    fn an_unregistered_function_fails_to_prepare() {
        let sql = "select id, nope(name) from users";
        let a = analysis(sql);
        assert_eq!(a.diagnostics.len(), 1, "{:?}", a.diagnostics);
        assert_eq!(a.diagnostics[0].message, "no such function: nope");
        assert_eq!((a.diagnostics[0].start, a.diagnostics[0].length), (sql.find("nope").unwrap(), 4));
    }

    #[test]
    fn a_scalar_subquery_is_non_null_only_when_it_always_finds_one_row() {
        let a = analysis("select (select count(*) from posts p where p.user_id = u.id) as n, (select max(title) from posts) as top, (select title from posts limit 1) as one, (select count(*) from posts group by user_id) as g, (select 1) as k from users u");
        assert_eq!(col(&a, "n"), ("integer".into(), false));
        assert_eq!(col(&a, "top"), ("text".into(), true));
        assert_eq!(col(&a, "one"), ("text".into(), true));
        assert_eq!(col(&a, "g"), ("integer".into(), true));
        assert_eq!(col(&a, "k"), ("integer".into(), false));
    }

    #[test]
    fn truth_values_are_marked_and_stay_integers() {
        let r = analyze_ddl(
            "select id = 1 as a, exists (select 1 from posts) as b, name like 'x%' as c, not (score > 1) as d, case when id > 1 then true end as e, coalesce(email is null, false) as f, email in ('a', 'b') as g, id as h, id + 1 as i, (select p.id > 1 from posts p limit 1) as j, iif(id > 1, 0, 1) as k from users",
            &ddl(),
            &Extras::default(),
        );
        assert!(r.analysis.diagnostics.is_empty(), "{:?}", r.analysis.diagnostics);
        assert_eq!(r.booleans, [true, true, true, true, true, true, true, false, false, true, false]);
        // The storage class is what the value is: an integer, 0 or 1.
        assert_eq!(col(&r.analysis, "a"), ("integer".into(), false));
        assert_eq!(col(&r.analysis, "b"), ("integer".into(), false));
        // Through a subquery's column, and across every arm of a compound.
        let r = analyze_ddl("select t.x, t.y from (select id > 1 as x, id as y from users) t", &ddl(), &Extras::default());
        assert_eq!(r.booleans, [true, false]);
        let r = analyze_ddl("select id > 1 as x from users union all select exists (select 1 from posts) union all select 0", &ddl(), &Extras::default());
        assert_eq!(r.booleans, [false]);
        let r = analyze_ddl("select id > 1 as x from users union all select email is null from users", &ddl(), &Extras::default());
        assert_eq!(r.booleans, [true]);
        // RETURNING, and a statement that does not prepare.
        let r = analyze_ddl("insert into users (name) values ('a') returning id, id > 0 as positive", &ddl(), &Extras::default());
        assert_eq!(r.booleans, [false, true]);
        assert!(analyze_ddl("select nope from users", &ddl(), &Extras::default()).booleans.is_empty());
    }

    #[test]
    fn sqlite_only_syntax_parses() {
        let a = analysis("select id from users limit :n, :m");
        assert_eq!(param(&a, "n"), "integer");
        assert_eq!(analysis("insert or replace into users (id, name) values (:id, :name) returning id").cardinality, "one");
    }
}
