//! The SQLite analysis, over SQLite's own resolved tree ([`crate::sqlite_tree`], produced by
//! `csrc/reylo_sqlite.c` from a patched amalgamation). SQLite resolves the names, the join sides,
//! every parameter's context and the DML target columns itself, and reports each expression's
//! affinity. What SQLite has no facts for stays here, as explicit tables: the result type and
//! nullability of the built-in functions, the type a function gives its arguments, the arithmetic
//! rule, and the row-count proof from the unique keys. The host runs two queries, one for the
//! schema DDL and one for the functions its SQLite has; everything else happens inside the engine.
//!
//! Two entries share the code: [`analyze`], replayed through the [`Host`] like every dialect, and
//! [`analyze_ddl`], for a host that holds a live connection and runs [`DDL_QUERY`] and
//! [`FUNCTIONS_QUERY`] on it itself, with [`Extras`] naming the scalar functions it registers there.

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

/// The functions the host's SQLite has, which decide what a statement may call: SQLite builds
/// differ in them (the math functions, `soundex`, a newer release's additions), and the embedded
/// one has every optional family, so it alone would accept a call the host then refuses to run.
/// The replayed entry asks the host for it; a host with a connection of its own runs it there and
/// hands the rows to [`Extras::host`] through [`host_functions`]. A SQLite that cannot answer (an
/// old one whose pragma lacks these columns, or a service that refuses the pragma) is taken to
/// have what the embedded one has.
pub const FUNCTIONS_QUERY: &str = "select name, narg, type from pragma_function_list";

/// One row of [`FUNCTIONS_QUERY`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunction {
    pub name: String,
    /// The argument count as SQLite keeps it: -1 for any number.
    pub arity: i32,
    /// `s` for a scalar function, `a` for an aggregate, `w` for a window function.
    pub kind: char,
}

/// The rows of [`FUNCTIONS_QUERY`] as the host's function list; `None` when there is none to
/// read, since every SQLite has functions and an empty list would refuse every call.
pub fn host_functions(rows: &[Vec<serde_json::Value>]) -> Option<Vec<HostFunction>> {
    let listed: Vec<HostFunction> = rows
        .iter()
        .filter_map(|row| {
            let name = row.first().and_then(as_str)?.to_string();
            let arity = row.get(1).and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok())).unwrap_or(-1) as i32;
            let kind = row.get(2).and_then(as_str).and_then(|k| k.chars().next()).unwrap_or('s');
            Some(HostFunction { name, arity, kind })
        })
        .collect();
    (!listed.is_empty()).then_some(listed)
}

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

/// What a host says of the SQLite on its own connection: the functions it registers there, and
/// the functions that SQLite has ([`FUNCTIONS_QUERY`]; `None` when it cannot list them).
#[derive(Debug, Clone, Default)]
pub struct Extras {
    pub total: Vec<TotalFunction>,
    pub host: Option<Vec<HostFunction>>,
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

/// The replayed entry: the host answers [`DDL_QUERY`] and [`FUNCTIONS_QUERY`] (session-scoped,
/// asked in one round), the rest is [`run`].
pub fn analyze(sql: &str, lexed: &Lexed, host: &Host) -> Lazy<Analysis> {
    let schema = host.query(DDL_QUERY, Vec::new(), Scope::Session);
    let functions = host.query(FUNCTIONS_QUERY, Vec::new(), Scope::Session);
    let (rows, functions) = (schema?, functions?);
    let ddl: Vec<String> = match rows {
        Ok(rows) => rows.iter().filter_map(|r| r.first().and_then(as_str)).map(String::from).collect(),
        Err(e) => return Some(crate::postgres::bare(sql, lexed, Dialect::Sqlite, vec![diag(format!("Cannot read the schema: {}", e.message), 0, 1, 5)])),
    };
    // A host that cannot list its functions keeps the embedded SQLite's.
    let extras = Extras { total: Vec::new(), host: functions.ok().and_then(|rows| host_functions(&rows)) };
    Some(run(sql, lexed, &ddl, &extras).analysis)
}

/// The direct entry, for a host with a connection of its own: `ddl` is the `sql` column of
/// [`DDL_QUERY`] run there, `extras` the functions it registers there. No round trip; a lex
/// error, a prepare error (SQLite's own message, at its offset) and a replay failure come back as
/// diagnostics, as on the replayed path.
pub fn analyze_ddl(sql: &str, ddl: &[String], extras: &Extras) -> SqliteAnalysis {
    crate::stack::run(|| analyze_ddl_here(sql, ddl, extras))
}

fn analyze_ddl_here(sql: &str, ddl: &[String], extras: &Extras) -> SqliteAnalysis {
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
    let listed: Option<Vec<tree::Listed>> = extras.host.as_ref().map(|host| host.iter().map(|f| (f.name.as_str(), f.arity, f.kind)).collect());
    let t = match tree::analyze(&replay, &numbered.sql, &functions, listed.as_deref()) {
        Ok(t) => t,
        Err(message) => return failed(crate::postgres::bare(sql, lexed, Dialect::Sqlite, vec![diag(message, 0, sql.trim_end().len(), 6)])),
    };
    let mut diagnostics = Vec::new();
    if let Some(err) = &t.error {
        let start = usize::try_from(err.offset).map_or(0, |o| numbered.to_original(o)).min(sql.len());
        let length = match err.length {
            Some(length) if err.offset >= 0 => length,
            _ if err.offset >= 0 => token_len(&sql[start..]),
            _ => sql.trim_end().len(),
        };
        let message = match (&err.missing_function, &err.missing_schema) {
            // The embedded SQLite has the function; the SQLite the statement will run on does not.
            (Some(name), _) => match &err.view {
                Some(view) => format!("no such function: {name} on this database's SQLite (the view {view} calls it)"),
                None => format!("no such function: {name} on this database's SQLite"),
            },
            // The replay holds the database file's own schema. An ATTACH is an act of the
            // application's connection at run time, and nothing in the file names it.
            (None, Some(schema)) => format!("{}: the database has no schema `{schema}`. A database the application attaches at run time (ATTACH ... AS {schema}) is not in the file the type check reads, so a table in it cannot be typed", err.message),
            (None, None) => err.message.clone(),
        };
        diagnostics.push(diag(message, start, length, 6));
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
        structured: Vec::new(),
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
/// `coalesce` or `ifnull`: the caller means to pass NULL for them. The test may be of the column of
/// a WITH query or a subquery that selects the parameter (`with input as (select :p as p) … where
/// input.p is null`), since each parameter appears once in a statement and a value tested and read
/// again is selected once in a WITH.
fn guarded_params(t: &Tree) -> HashSet<String> {
    /// Every FROM item's subquery, a WITH query's too, by the item's cursor: SQLite numbers the
    /// cursors of one statement apart.
    fn subqueries<'a>(sel: &'a Sel, out: &mut HashMap<i64, &'a Sel>) {
        for f in &sel.from {
            if let Some(sub) = &f.subquery {
                out.insert(f.cursor, sub);
                subqueries(sub, out);
            }
        }
        for n in sel.columns.iter().map(|c| &c.expr).chain(sel.where_.iter()).chain(sel.having.iter()) {
            nested(n, out);
        }
        if let Some(prior) = &sel.prior {
            subqueries(prior, out);
        }
    }
    fn nested<'a>(n: &'a Node, out: &mut HashMap<i64, &'a Sel>) {
        if let Some(sel) = &n.select {
            subqueries(sel, out);
        }
        for child in n.left.iter().chain(n.right.iter()).map(|b| b.as_ref()).chain(n.list.iter()).chain(n.args.iter()) {
            nested(child, out);
        }
    }
    /// The parameter an operand is, under any casts, or that the column of a subquery selects. A
    /// compound subquery's column is any of its arms', so it selects no one parameter.
    fn variable(x: Option<&Node>, subs: &HashMap<i64, &Sel>, depth: usize) -> Option<String> {
        let x = x?;
        match x.op.as_str() {
            "variable" => x.name.clone(),
            "cast" => variable(x.left.as_deref(), subs, depth),
            "column" if depth < 32 => {
                let sel = subs.get(&x.cursor).filter(|sel| sel.prior.is_none() && sel.rows.is_empty())?;
                variable(sel.columns.get(usize::try_from(x.index).ok()?).map(|c| &c.expr), subs, depth + 1)
            }
            _ => None,
        }
    }
    fn walk(n: &Node, subs: &HashMap<i64, &Sel>, out: &mut HashSet<String>) {
        match n.op.as_str() {
            "isnull" | "notnull" | "is" | "is not" => {
                out.extend(variable(n.left.as_deref(), subs, 0));
                out.extend(variable(n.right.as_deref(), subs, 0));
            }
            "function" if matches!(n.name.as_deref(), Some("coalesce" | "ifnull")) => out.extend(variable(n.args.first(), subs, 0)),
            _ => {}
        }
        for child in n.left.iter().chain(n.right.iter()).map(|b| b.as_ref()).chain(n.list.iter()).chain(n.args.iter()) {
            walk(child, subs, out);
        }
        if let Some(sel) = &n.select {
            walk_select(sel, subs, out);
        }
    }
    fn walk_select(sel: &Sel, subs: &HashMap<i64, &Sel>, out: &mut HashSet<String>) {
        for n in sel.columns.iter().map(|c| &c.expr).chain(sel.rows.iter().flatten()) {
            walk(n, subs, out);
        }
        for n in sel.where_.iter().chain(sel.having.iter()).chain(sel.group_by.iter()).chain(sel.order_by.iter()).chain(sel.limit.iter()).chain(sel.offset.iter()) {
            walk(n, subs, out);
        }
        for f in &sel.from {
            if let Some(sub) = &f.subquery {
                walk_select(sub, subs, out);
            }
            for a in &f.args {
                walk(a, subs, out);
            }
        }
        if let Some(prior) = &sel.prior {
            walk_select(prior, subs, out);
        }
    }
    let mut subs = HashMap::new();
    if let Some(sel) = &t.statement {
        subqueries(sel, &mut subs);
    }
    if let Some(dml) = &t.dml {
        for n in dml.values.iter().flatten().chain(dml.set.iter().map(|s| &s.expr)).chain(dml.where_.iter()) {
            nested(n, &mut subs);
        }
    }
    let mut out = HashSet::new();
    if let Some(sel) = &t.statement {
        walk_select(sel, &subs, &mut out);
    }
    if let Some(dml) = &t.dml {
        for n in dml.values.iter().flatten().chain(dml.set.iter().map(|s| &s.expr)).chain(dml.where_.iter()) {
            walk(n, &subs, &mut out);
        }
        if let Some(u) = &dml.upsert {
            for n in u.set.iter().map(|s| &s.expr).chain(u.where_.iter()) {
                walk(n, &subs, &mut out);
            }
        }
    }
    for c in t.returning.iter().flatten() {
        walk(&c.expr, &subs, &mut out);
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

/// The words of a statement as SQLite's tokenizer splits them: a name or a keyword, and each
/// other character alone; comments are none. A quoted name (`"a b"`, `` `a` ``, `[a]`, `'a'`) is
/// one word without its quotes, and is marked, since it is never a keyword.
fn words(sql: &str) -> Vec<(String, bool)> {
    let chars: Vec<char> = sql.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '-' && chars.get(i + 1) == Some(&'-') {
            i += chars[i..].iter().position(|c| *c == '\n').unwrap_or(chars.len() - i);
        } else if c == '/' && chars.get(i + 1) == Some(&'*') {
            i += 2;
            while i < chars.len() && !(chars[i] == '*' && chars.get(i + 1) == Some(&'/')) {
                i += 1;
            }
            i += 2;
        } else if matches!(c, '"' | '`' | '\'' | '[') {
            // A doubled quote inside the name is the quote itself; `[` ends at the first `]`.
            let close = if c == '[' { ']' } else { c };
            let mut name = String::new();
            i += 1;
            while i < chars.len() {
                if chars[i] == close {
                    if close != ']' && chars.get(i + 1) == Some(&close) {
                        i += 1;
                    } else {
                        break;
                    }
                }
                name.push(chars[i]);
                i += 1;
            }
            i += 1;
            out.push((name, true));
        } else if c.is_alphanumeric() || c == '_' || c == '$' || !c.is_ascii() {
            let end = chars[i..].iter().position(|c| !(c.is_alphanumeric() || *c == '_' || *c == '$' || !c.is_ascii())).map_or(chars.len(), |n| i + n);
            out.push((chars[i..end].iter().collect(), false));
            i = end;
        } else {
            out.push((c.to_string(), false));
            i += 1;
        }
    }
    out
}

/// A `CREATE TRIGGER` statement's timing (`before`, `after`, `instead`), event (`insert`,
/// `update`, `delete`) and table, the table in ASCII lowercase, as SQLite compares names; `None`
/// for any other statement.
fn trigger_header(sql: &str) -> Option<(String, String, String)> {
    // Most of a schema is tables and indexes, which need no reading here.
    if !sql.as_bytes().windows(7).any(|w| w.eq_ignore_ascii_case(b"trigger")) {
        return None;
    }
    let words = words(sql);
    let keyword = |i: usize| words.get(i).filter(|(_, quoted)| !quoted).map(|(w, _)| w.to_ascii_lowercase());
    // Steps over the next word when it is one of the keywords `want`.
    let take = |i: &mut usize, want: &[&str]| {
        let hit = keyword(*i).is_some_and(|w| want.contains(&w.as_str()));
        if hit {
            *i += 1;
        }
        hit
    };
    let mut i = 0;
    if !take(&mut i, &["create"]) {
        return None;
    }
    take(&mut i, &["temp", "temporary"]);
    if !take(&mut i, &["trigger"]) {
        return None;
    }
    if take(&mut i, &["if"]) {
        take(&mut i, &["not"]);
        take(&mut i, &["exists"]);
    }
    // The trigger's name, with its schema when it has one.
    i += 1;
    if take(&mut i, &["."]) {
        i += 1;
    }
    let timing = if take(&mut i, &["before"]) {
        "before"
    } else if take(&mut i, &["after"]) {
        "after"
    } else if take(&mut i, &["instead"]) {
        take(&mut i, &["of"]);
        "instead"
    } else {
        // "If no timing is specified, BEFORE is the default."
        "before"
    };
    let event = keyword(i)?;
    // `UPDATE OF a, b ON t`: the first ON that is no quoted column name.
    let on = (i..words.len()).find(|j| keyword(*j).as_deref() == Some("on"))?;
    let mut table = &words.get(on + 1)?.0;
    if keyword(on + 2).as_deref() == Some(".") {
        table = &words.get(on + 3)?.0;
    }
    Some((timing.to_string(), event, table.to_ascii_lowercase()))
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
            if !arm.columns.get(i).is_some_and(|c| self.truth(&c.expr, &inner)) || !arm.rows.iter().all(|row| row.get(i).is_some_and(|e| self.truth(e, &inner))) {
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
    /// Every row of a multi-row VALUES is an arm.
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
            .flat_map(|arm| {
                let mut inner = scope.clone();
                inner.push(&arm.from);
                let outer = self.level.replace(level_of(arm, self.extras));
                let rows = std::iter::once(arm.columns.get(i).map(|c| &c.expr)).chain(arm.rows.iter().map(|row| row.get(i)));
                let facts: Vec<Fact> = rows.map(|e| e.map(|e| self.type_of(e, &inner)).unwrap_or_else(Fact::unknown)).collect();
                self.level.replace(outer);
                facts
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
            "length" | "instr" | "unixepoch" | "json_array_length" | "json_valid" | "octet_length" | "sign" => done("integer", Nulls::IfArgs),
            // `unicode('')` is NULL: an empty string has no first character.
            "unicode" => done("integer", Nulls::Always),
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
            // `soundex(NULL)` is '?000'.
            "hex" | "quote" | "typeof" | "soundex" => done("text", Nulls::Never),
            "sqlite_version" | "sqlite_source_id" | "current_date" | "current_time" | "current_timestamp" => done("text", Nulls::Never),
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
        // A parameter that is the whole value written to a column may be null when that column
        // may, and no NOT NULL column it also fills forbids it.
        let mut fills: HashMap<String, bool> = HashMap::new();
        let mut fill = |v: &Node, target: &Option<Fact>| {
            if let (true, Some(name), Some(target)) = (v.op == "variable", v.name.as_ref(), target) {
                *fills.entry(name.clone()).or_insert(true) &= !target.non_null;
            }
        };
        if let Some(sel) = &t.statement {
            let targets: Vec<Option<Fact>> = match &t.dml {
                // The SELECT or the VALUES rows that feed an INSERT: their columns type from the
                // target columns.
                Some(dml) if dml.kind == "insert" => dml.columns.iter().map(|c| c.as_deref().and_then(|c| self.table_column(&dml.table, c))).collect(),
                _ => Vec::new(),
            };
            // Every row of a VALUES list writes its values whole, as the one-row form below does;
            // a SELECT's column is no such value, as on Postgres.
            if sel.values && sel.prior.is_none() {
                for (i, v) in sel.columns.iter().map(|c| &c.expr).enumerate().chain(sel.rows.iter().flat_map(|row| row.iter().enumerate())) {
                    if let Some(target) = targets.get(i) {
                        fill(v, target);
                    }
                }
            }
            self.walk_select(sel, &Vec::new(), &targets, &mut found);
        }
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
        let mut typed: HashMap<&str, &Fact> = HashMap::new();
        for (name, fact) in found.iter().filter(|(_, f)| !matches!(f.ty, Type::Unknown)) {
            typed.entry(name.as_str()).or_insert(fact);
        }
        lexed
            .params
            .iter()
            .enumerate()
            .map(|(k, name)| {
                let numbered = format!("?{}", k + 1);
                let fact = typed.get(numbered.as_str()).map(|f| (*f).clone()).unwrap_or_else(Fact::unknown);
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
        let targets = if targets.is_empty() && (sel.prior.is_some() || !sel.rows.is_empty()) {
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
        for (i, e) in sel.columns.iter().map(|c| &c.expr).enumerate().chain(sel.rows.iter().flat_map(|row| row.iter().enumerate())) {
            self.walk(e, Ctx::Typed(targets.get(i).cloned().flatten()), &inner, found);
        }
        for f in &sel.from {
            if let Some(sub) = &f.subquery {
                self.walk_select(sub, &inner, &[], found);
            }
            if let Some(on) = &f.on {
                self.walk(on, Ctx::Bool, &inner, found);
            }
            // `json_each` and `json_tree` read JSON text and a path (`IN (SELECT value FROM json_each(:ids))`).
            let json = f.tab_func && matches!(f.table.as_deref(), Some("json_each" | "json_tree"));
            for (i, a) in f.args.iter().enumerate() {
                self.walk(a, if json && i < 2 { Ctx::Typed(Some(Fact::new("text", true))) } else { Ctx::None }, &inner, found);
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
                Ctx::Typed(None) | Ctx::Row(_) | Ctx::None => Fact::unknown(),
                Ctx::Bool => Fact::new("integer", false),
            };
            found.push((n.name.clone().unwrap_or_default(), fact));
            return;
        }
        let sibling = |other: &Option<Box<Node>>| other.as_deref().map(|o| self.type_of(o, scope));
        let child_ctx = |slot: Slot| -> Ctx {
            match n.op.as_str() {
                "=" | "!=" | "<" | "<=" | ">" | ">=" | "is" | "is not" => {
                    let (this, other) = match slot {
                        Slot::Left => (&n.left, &n.right),
                        Slot::Right => (&n.right, &n.left),
                        _ => return Ctx::None,
                    };
                    // Row values compare element by element, so each element takes the type of
                    // the one it is compared with: `(created_at, id) < (:created_at, :id)`, and a
                    // subquery's columns: `(created_at, id) = (SELECT :at, :id)`.
                    match (this.as_deref(), other.as_deref().and_then(|o| self.row(o, scope))) {
                        (Some(t), Some(row)) if matches!(t.op.as_str(), "vector" | "select") => Ctx::Row(row),
                        _ => Ctx::Typed(sibling(other)),
                    }
                }
                "in" => match slot {
                    // `(created_at, id) IN (SELECT …)`: SQLite writes a list of row values as
                    // a VALUES subquery, which the left side types (see the select walk below).
                    Slot::Left if is_vector(&n.left) => n.select.as_deref().map_or(Ctx::None, |s| Ctx::Row(self.select_columns(s, scope))),
                    Slot::Left => Ctx::Typed(n.list.first().map(|x| self.type_of(x, scope)).or_else(|| n.select.as_deref().map(|s| self.select_column(s, 0, scope)))),
                    Slot::List(_) => Ctx::Typed(sibling(&n.left)),
                    _ => Ctx::None,
                },
                "between" => match slot {
                    Slot::Left if is_vector(&n.left) => Ctx::Row(self.rows(&n.list.iter().collect::<Vec<_>>(), scope)),
                    Slot::Left => Ctx::Typed(n.list.first().map(|x| self.type_of(x, scope))),
                    Slot::List(_) => match n.left.as_deref().and_then(|l| self.row(l, scope)) {
                        Some(row) if is_vector(&n.left) => Ctx::Row(row),
                        _ => Ctx::Typed(sibling(&n.left)),
                    },
                    _ => Ctx::None,
                },
                // An element of a row value takes its place's type.
                "vector" => match (slot, &ctx) {
                    (Slot::List(i), Ctx::Row(row)) => Ctx::Typed(row.get(i).cloned().flatten()),
                    _ => Ctx::None,
                },
                // One column of a row subquery, as `UPDATE … SET (a, b) = (SELECT …)` writes it.
                "select_column" => match (slot, &ctx) {
                    (Slot::Left, Ctx::Typed(Some(f))) => {
                        let mut row = vec![None; n.index.max(0) as usize + 1];
                        row[n.index.max(0) as usize] = Some(f.clone());
                        Ctx::Row(row)
                    }
                    _ => Ctx::None,
                },
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
                // So does a COLLATE, which names how the value compares and leaves it as it is.
                "pos" | "collate" => ctx.clone(),
                "and" | "or" | "not" => Ctx::Bool,
                "case" => match slot {
                    // A WHEN compares with the base when there is one, else it is a condition; a
                    // THEN or ELSE shares the other results' type, or failing that the type the
                    // CASE stands in (`name = CASE WHEN … THEN :a ELSE :b END`).
                    Slot::List(i) if i % 2 == 0 && i + 1 < n.list.len() => match &n.left {
                        Some(base) if base.op == "vector" => self.row(base, scope).map_or(Ctx::None, Ctx::Row),
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
                    // A base compares with each WHEN value (`CASE :kind WHEN 'a' THEN …`), a row
                    // value element by element.
                    Slot::Left if is_vector(&n.left) => {
                        let values: Vec<&Node> = n.list.iter().enumerate().filter(|(j, _)| j % 2 == 0 && j + 1 < n.list.len()).map(|(_, x)| x).collect();
                        Ctx::Row(self.rows(&values, scope))
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
            // A subquery's columns take the types they are compared with or stand in: `id IN
            // (SELECT :p)`, `id = (SELECT :p)`, `(created_at, id) IN (VALUES (:at, :id))`.
            let targets: Vec<Option<Fact>> = match (n.op.as_str(), &ctx) {
                ("select", Ctx::Row(row)) => row.clone(),
                ("select", Ctx::Typed(Some(f))) => vec![Some(f.clone())],
                ("in", _) => match n.left.as_deref() {
                    Some(l) if l.op == "vector" => self.row(l, scope).unwrap_or_default(),
                    Some(l) => vec![Some(self.type_of(l, scope))],
                    None => Vec::new(),
                },
                _ => Vec::new(),
            };
            self.walk_select(s, scope, &targets, found);
        }
    }

    /// A row value's elements, one fact per place: a vector's elements, or a subquery's columns.
    fn row(&self, n: &Node, scope: &ScopeStack) -> Option<Vec<Option<Fact>>> {
        match n.op.as_str() {
            "vector" => Some(n.list.iter().map(|x| Some(self.type_of(x, scope))).collect()),
            "select" => n.select.as_deref().map(|s| self.select_columns(s, scope)),
            _ => None,
        }
    }

    fn select_columns(&self, sel: &Sel, scope: &ScopeStack) -> Vec<Option<Fact>> {
        (0..sel.columns.len()).map(|i| Some(self.select_column(sel, i, scope))).collect()
    }

    /// The type at each place of several row values (BETWEEN's bounds, a CASE's WHEN values):
    /// the first that has one there.
    fn rows(&self, values: &[&Node], scope: &ScopeStack) -> Vec<Option<Fact>> {
        let mut out: Vec<Option<Fact>> = Vec::new();
        for row in values.iter().filter_map(|v| self.row(v, scope)) {
            for (i, f) in row.into_iter().enumerate() {
                if i >= out.len() {
                    out.push(None);
                }
                if out[i].as_ref().is_none_or(|known| matches!(known.ty, Type::Unknown)) {
                    out[i] = f;
                }
            }
        }
        out
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
            "like" | "glob" | "regexp" | "match" | "upper" | "lower" | "trim" | "ltrim" | "rtrim" | "length" | "hex" | "unicode" | "quote" | "instr" | "replace" | "soundex"
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
        // A constant LIMIT of a row or more keeps the one row, and so does an OFFSET of 0; a
        // parameter may be 0.
        let keeps = |n: &Node, least: i64| integer(n).is_some_and(|n| n >= least);
        if sel.limit.as_ref().is_some_and(|l| !keeps(l, 1)) || sel.offset.as_ref().is_some_and(|o| integer(o) != Some(0)) {
            return Err("a LIMIT or an OFFSET can remove the row (LIMIT 0, OFFSET 1)".into());
        }
        self.one_before_limit(sel)
    }

    /// Exactly one row before the statement's LIMIT and OFFSET apply.
    fn one_before_limit(&self, sel: &Sel) -> Result<(), String> {
        if sel.prior.is_some() {
            return Err("a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows".into());
        }
        if sel.values && !sel.rows.is_empty() {
            return Err("a VALUES list returns a row per row it lists".into());
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
        // SQLite marks the SELECT an aggregate belongs to: one whose arguments read only columns
        // of a query around this one (`count(a.id)` in a subquery over `b`) is that query's, and
        // leaves this one a row per row of its FROM.
        if sel.aggregate && sel.columns.iter().any(|c| has_aggregate(&c.expr)) {
            return Ok(());
        }
        if sel.from.is_empty() && sel.where_.is_none() {
            return Ok(());
        }
        // A FROM of exactly one row, as `cardinality` proves it: never empty, and one row at most.
        if !sel.from.is_empty() && self.never_empty(sel) && self.arm_at_most_one(sel).is_ok() {
            return Ok(());
        }
        Err("a FROM returns a row per row it matches, and only an aggregate with no GROUP BY collapses them to one".into())
    }

    /// Whether the FROM returns a row whatever the tables hold: every item is a subquery or a WITH
    /// query of exactly one row, or sits on the nullable side of a LEFT JOIN, which keeps the row
    /// of its other side; and no WHERE term can fail but one from an outer join's ON, which
    /// removes no row. An inner join's ON is a WHERE term here, so `ON 1 = 1` is one that holds.
    /// A VALUES in a FROM is any number of rows: SQLite reads a list of several rows as one such
    /// item, which does not say how many it lists.
    fn never_empty(&self, sel: &Sel) -> bool {
        let holds = |t: &Node| match (t.op.as_str(), t.left.as_deref().and_then(integer), t.right.as_deref().and_then(integer)) {
            ("=", Some(l), Some(r)) => l == r,
            _ => integer(t).is_some_and(|n| n != 0),
        };
        sel.from.iter().all(|f| !f.right && (f.outer || f.subquery.as_ref().is_some_and(|sub| !sub.values && self.exactly_one(sub).is_ok())))
            && conjuncts(sel.where_.as_ref()).iter().all(|t| t.outer_on || holds(t))
    }

    fn at_most_one(&self, sel: &Sel) -> Result<(), String> {
        if sel.limit.as_ref().and_then(integer).is_some_and(|n| (0..=1).contains(&n)) {
            return Ok(());
        }
        // Any other LIMIT, and an OFFSET, only remove rows: the bound is the one the statement has
        // without them.
        if !sel.values && self.one_before_limit(sel).is_ok() {
            return Ok(());
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
    "json_quote", "json_pretty", "json_type", "hex", "quote", "typeof", "soundex", "zeroblob", "json_extract", "jsonb_extract", "->", "->>", "acos", "acosh", "asin", "asinh",
    "atan", "atan2", "atanh", "ceil", "ceiling", "cos", "cosh", "degrees", "exp", "floor", "ln", "log", "log10", "log2", "mod", "pow", "power", "radians", "sin",
    "sinh", "sqrt", "tan", "tanh", "trunc", "pi",
];

/// An integer literal's value.
fn integer(n: &Node) -> Option<i64> {
    if n.op == "integer" { n.value.as_deref().and_then(|v| v.parse().ok()) } else { None }
}

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
fn is_vector(n: &Option<Box<Node>>) -> bool {
    n.as_deref().is_some_and(|n| n.op == "vector")
}

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
    /// A row value's place: element i takes fact i (`(created_at, id) < (:created_at, :id)`).
    Row(Vec<Option<Fact>>),
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

    #[test]
    fn a_statement_as_deep_as_the_engine_reads_is_typed_and_one_level_past_it_is_an_error_at_the_bound() {
        // From a thread with a small stack, as a host may call from: the embedded SQLite's
        // recursion, the reading of its tree and the walks over it run on the engine's stack.
        let ddl = vec!["create table users (id integer primary key, name text not null, nick text)".to_string()];
        let small = |sql: String| std::thread::scope(|s| std::thread::Builder::new().stack_size(256 << 10).spawn_scoped(s, || analyze_ddl(&sql, &ddl, &Extras::default()).analysis).unwrap().join().unwrap());
        // SQLite's own bound is 1,000; the engine's SQLite takes half of it (`csrc/reylo_sqlite.c`).
        for (shape, n) in [("sum", 500), ("and_or", 498), ("case", 498), ("coalesce", 499), ("subquery", 29)] {
            let at = small(crate::stack::nested(shape, n));
            assert!(at.diagnostics.is_empty() && at.kind == "select", "{shape} at {n}: {:?}", at.diagnostics);
            let past = small(crate::stack::nested(shape, n + 1));
            assert_eq!(past.diagnostics.iter().map(|d| d.message.as_str()).collect::<Vec<_>>(), ["Expression tree is too large (maximum depth 500)"], "{shape} at {}", n + 1);
        }
        // A list does not nest: SQLite's OR is a tree, which its bound counts; an IN list is not.
        let list = (0..5_000).map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
        assert!(small(format!("select id from users where id in ({list})")).diagnostics.is_empty());
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
        // Each parameter appears once, so a value tested and read again is selected once in a
        // WITH, and the test of its column is the test of the parameter, however it is named.
        assert!(nullable_param("with input as (select cast(:p as integer) as p) select id from users, input where (input.p is null or score > input.p)", "p"));
        assert!(nullable_param("with input as (select :p as p) select id from users, input i where (p is null or score > i.p)", "p"));
        assert!(nullable_param("select id from users, (select :p as p) s where s.p is null or score > s.p", "p"));
        assert!(nullable_param("with input as (select :p as p) select id from users, input where score = coalesce(input.p, score)", "p"));
        assert!(!nullable_param("with input as (select :p as p) select id from users, input where score > input.p", "p"));
        assert!(!nullable_param("with input as (select :p as p union all select 1) select id from users, input where input.p is null", "p"));
        assert!(nullable_param("select id from users where cast(:p as integer) is null", "p"));
        let sql = "select id from users where (:p is null or score > :p)";
        let repeated = analysis(sql);
        assert_eq!(repeated.diagnostics.iter().map(|d| (d.code, d.start, d.length)).collect::<Vec<_>>(), [(3, sql.rfind(":p").unwrap(), 2)]);
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
        let lower = Extras { total: vec![TotalFunction { name: "lower".into(), arity: 1, returns: oid::TEXT }], host: None };
        let nullable = |extras: &Extras| col(&analyze_ddl("select email from users where lower(email) = :e", &ddl(), extras).analysis, "email").1;
        assert!(!nullable(&Extras::default()));
        assert!(nullable(&lower));
    }

    #[test]
    fn a_table_whose_check_calls_a_function_the_host_lacks_still_replays() {
        // sqlite-jsonschema's function, which the application's connection loads and the type
        // check's does not: CREATE TABLE resolves the CHECK, and the table was left out.
        let ddl = vec!["create table docs (id integer primary key, doc text check (doc is null or jsonschema_matches('{\"type\":\"object\"}', doc)))".to_string()];
        let without = Extras { total: vec![], host: Some(vec![HostFunction { name: "lower".into(), arity: 1, kind: 's' }]) };
        for extras in [Extras::default(), without.clone()] {
            let read = analyze_ddl("select id, doc from docs", &ddl, &extras).analysis;
            assert!(read.diagnostics.is_empty(), "{:?}", read.diagnostics);
            assert_eq!(read.columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["id", "doc"]);
            let write = analyze_ddl("insert into docs (doc) values (:doc) returning id", &ddl, &extras).analysis;
            assert!(write.diagnostics.is_empty(), "{:?}", write.diagnostics);
            assert_eq!(param(&write, "doc"), "text");
        }
        // A statement that calls the function itself is still refused where the host lacks it.
        let call = analyze_ddl("select jsonschema_matches('{}', doc) as m from docs", &ddl, &without).analysis;
        assert!(!call.diagnostics.is_empty());
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
    fn a_row_value_types_each_parameter_by_the_element_it_is_compared_with() {
        // A keyset page: each element of a row value takes the type of its partner, in every
        // comparison SQLite takes a row value in, a subquery's columns and a VALUES list too.
        let ddl = vec!["create table users (id integer primary key, name text not null, created_at text not null) strict".to_string()];
        let types = |sql: &str| {
            let a = analyze_ddl(sql, &ddl, &Extras::default()).analysis;
            assert!(a.diagnostics.is_empty(), "{sql}: {:?}", a.diagnostics);
            a.params.iter().map(|p| format!("{}: {}{}", p.name, scalar_name(&p.ty).unwrap_or("unknown"), if p.nullable { " | null" } else { "" })).collect::<Vec<_>>().join(", ")
        };
        let page = "select id, name, created_at from users where (created_at, id) < (:created_at, :id) order by created_at desc, id desc limit 20";
        assert_eq!(types(page), "created_at: text, id: integer");
        for op in ["=", "<>", "!=", "<", "<=", ">", ">=", "is", "is not"] {
            assert_eq!(types(&format!("select id from users where (created_at, id) {op} (:at, :id)")), "at: text, id: integer", "{op}");
            assert_eq!(types(&format!("select id from users where (:at, :id) {op} (created_at, id)")), "at: text, id: integer", "{op} reversed");
        }
        assert_eq!(types("select id from users where (created_at, :id) < (:at, id)"), "id: integer, at: text");
        assert_eq!(types("select id from users where (created_at, id) between (:lo_at, :lo_id) and (:hi_at, :hi_id)"), "lo_at: text, lo_id: integer, hi_at: text, hi_id: integer");
        assert_eq!(types("select id from users where (:at, :id) between (created_at, id) and ('9999', 0)"), "at: text, id: integer");
        assert_eq!(types("select id from users where (created_at, id) in (select :at, :id)"), "at: text, id: integer");
        assert_eq!(types("select id from users where (:at, :id) in (select created_at, id from users)"), "at: text, id: integer");
        assert_eq!(types("select id from users where (created_at, id) in ((:a1, :i1), (:a2, :i2))"), "a1: text, i1: integer, a2: text, i2: integer");
        assert_eq!(types("select id from users where (created_at, id) = (select :at, :id)"), "at: text, id: integer");
        assert_eq!(types("select case (created_at, id) when (:at, :id) then 1 else 0 end as hit from users"), "at: text, id: integer");
        assert_eq!(types("select case (:at, :id) when (created_at, id) then 1 else 0 end as hit from users"), "at: text, id: integer");
        assert_eq!(types("update users set (name, created_at) = (select :name, :at) where id = :id"), "name: text, at: text, id: integer");
        // One column of a subquery takes the type it is compared with, as a row's columns do.
        assert_eq!(types("select id from users where id = (select :p)"), "p: integer");
        assert_eq!(types("select id from users where id in (select :p union all select 1)"), "p: integer");
    }

    #[test]
    fn json_each_reads_text() {
        // A list of values in one parameter: the JSON text and the path are text, as written or cast.
        let a = analysis("select id from users where id in (select value from json_each(:ids))");
        assert_eq!(param(&a, "ids"), "text");
        assert!(!a.params.iter().find(|p| p.name == "ids").unwrap().nullable);
        let a = analysis("select key, value from json_tree(:doc, :path)");
        assert_eq!(param(&a, "doc"), "text");
        assert_eq!(param(&a, "path"), "text");
        assert_eq!(param(&analysis("select value from json_each(cast(:ids as text))"), "ids"), "text");
        let a = analysis("select value from json_each(coalesce(:ids, '[]'))");
        assert_eq!(param(&a, "ids"), "text");
        assert!(a.params.iter().find(|p| p.name == "ids").unwrap().nullable);
        // A parameter beside the call keeps its own context.
        let a = analysis("select users.id from users, json_each(:ids) j where users.id = j.value and users.name = :n");
        assert_eq!(param(&a, "ids"), "text");
        assert_eq!(param(&a, "n"), "text");
    }

    #[test]
    fn a_limit_only_lowers_a_bound_and_a_from_of_one_row_items_is_one_row() {
        let rows = |sql: &str| analysis(sql).cardinality;
        // A LIMIT above one, one from a parameter and an OFFSET remove rows and add none.
        assert_eq!(rows("select * from users where id = :id limit 5"), "maybeOne");
        assert_eq!(rows("select * from users where id = :id limit :n offset :skip"), "maybeOne");
        assert_eq!(rows("select * from users where id = :id limit 2, 5"), "maybeOne");
        assert_eq!(rows("select * from users limit 5"), "many");
        assert_eq!(rows("select * from users limit -1"), "many");
        assert_eq!(rows("select count(*) as n from users limit 5"), "one");
        assert_eq!(rows("select count(*) as n from users limit 5 offset 0"), "one");
        assert_eq!(rows("select count(*) as n from users limit 0"), "maybeOne");
        assert_eq!(rows("select count(*) as n from users limit :n"), "maybeOne");
        assert_eq!(rows("select count(*) as n from users limit -1"), "maybeOne");
        assert_eq!(rows("select count(*) as n from users limit 5 offset 1"), "maybeOne");
        // A derived table and a WITH query of one row, alone and joined.
        assert_eq!(rows("select * from (select count(*) as n from users) s"), "one");
        assert_eq!(rows("with s as (select count(*) as n from users) select n from s"), "one");
        assert_eq!(rows("select a.n, b.m from (select count(*) as n from users) a, (select count(*) as m from posts) b"), "one");
        assert_eq!(rows("select a.n, b.m from (select count(*) as n from users) a join (select count(*) as m from posts) b on 1 = 1"), "one");
        assert_eq!(rows("select a.n, u.name from (select count(*) as n from users) a left join users u on u.id = :id"), "one");
        // A WHERE or an inner join's condition can drop the row, an unpinned side multiplies it,
        // and a FULL JOIN is two rows where its condition fails.
        assert_eq!(rows("select s.n from (select count(*) as n from users) s where s.n > :min"), "maybeOne");
        assert_eq!(rows("select a.n, b.m from (select count(*) as n from users) a join (select count(*) as m from posts) b on a.n = b.m"), "maybeOne");
        assert_eq!(rows("select a.n, b.m from (select count(*) as n from users) a join (select count(*) as m from posts) b on 1 = 2"), "maybeOne");
        assert_eq!(rows("select s.n from (select count(*) as n from users limit :n) s"), "maybeOne");
        assert_eq!(rows("select a.n, u.name from (select count(*) as n from users) a left join users u on u.name = :name"), "many");
        assert_eq!(rows("select a.n, u.name from users u left join (select count(*) as n from users) a on 1 where u.id = :id"), "maybeOne");
        assert_eq!(rows("select a.n, b.m from (select count(*) as n from users) a full join (select count(*) as m from posts) b on a.n = b.m + 1"), "many");
        assert_eq!(rows("select a.n, u.name from (select count(*) as n from users) a, users u"), "many");
        // An aggregate over columns of the query around a subquery is that query's: the subquery
        // is a row per row of `posts`, and none when it has none.
        for sql in [
            "select (select count(u.id) from posts where 0) as n from users u",
            "select (select count(u.id) from posts where 0 limit 5) as n from users u",
            "select (select s.n from (select count(u.id) as n from posts where 0) s) as n from users u",
        ] {
            assert!(col(&analysis(sql), "n").1, "the scalar subquery may find no row: {sql}");
        }
        assert!(!col(&analysis("select (select count(p.id) from posts p where 0 limit 5) as n from users u"), "n").1);
        assert!(!col(&analysis("select (select s.n from (select count(p.id) as n from posts p where 0) s) as n from users u"), "n").1);
        // SQLite reads a VALUES of several rows as a FROM item, which does not say how many.
        assert_eq!(rows("values (1), (2)"), "many");
        assert_eq!(rows("select v.column1 from (values (1), (2)) v"), "many");
        assert_eq!(rows("select a.n, v.column1 from (select count(*) as n from users) a, (values (1), (2)) v"), "many");
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
        let answers = HashMap::from([(format!("query:{DDL_QUERY}|[]"), crate::api::Answer::Rows { rows }), (format!("query:{FUNCTIONS_QUERY}|[]"), functions_answer(&[]))]);
        let host = Host::new(&answers);
        analyze(sql, &lex::lex(sql, Dialect::Sqlite).unwrap(), &host).expect("the schema and the functions were answered")
    }

    /// What a SQLite with none of the optional families lists, as [`FUNCTIONS_QUERY`]'s rows: the
    /// functions the tests call, and `extra` on top.
    fn listed(extra: &[(&str, i32, char)]) -> Vec<Vec<serde_json::Value>> {
        let core = [("count", 0, 'w'), ("count", 1, 'w'), ("max", 1, 'w'), ("max", -3, 's'), ("upper", 1, 's'), ("lower", 1, 's'), ("like", 2, 's'), ("like", 3, 's'), ("glob", 2, 's')];
        let more = [("coalesce", -4, 's'), ("iif", -4, 's'), ("json_extract", -1, 's'), ("->>", 2, 's'), ("current_timestamp", 0, 's'), ("row_number", 0, 'w'), ("group_concat", 1, 'w'), ("abs", 1, 's')];
        core.iter().chain(more.iter()).chain(extra.iter()).map(|(name, narg, kind)| vec![serde_json::json!(name), serde_json::json!(narg), serde_json::json!(kind.to_string())]).collect()
    }

    fn functions_answer(extra: &[(&str, i32, char)]) -> crate::api::Answer {
        crate::api::Answer::Rows { rows: listed(extra) }
    }

    /// The direct entry on a host whose SQLite lists [`listed`]'s functions.
    fn on_host(sql: &str, schema: &[String], extra: &[(&str, i32, char)], total: Vec<TotalFunction>) -> Analysis {
        analyze_ddl(sql, schema, &Extras { total, host: host_functions(&listed(extra)) }).analysis
    }

    #[test]
    fn names_fold_ascii_case_as_sqlite_folds_them() {
        let schema: Vec<String> = [
            "create table Books (id integer primary key, Title text not null, ISBN text unique, note text)",
            "create table \"\u{c9}t\u{e9}\" (id integer primary key, v text)",
            "create table wr (K text primary key, v text) without rowid",
            "create table rt (RowId integer, v text)",
            "create table \"my table\" (id integer primary key, v text)",
            "create trigger \"skip one\" before insert on \"BOOKS\" begin select raise(ignore) where new.title = 'x'; end",
            "create trigger skip_two before insert on \"my table\" begin select raise(ignore) where new.v = 'x'; end",
            "create trigger [later] after update of \"on\", v on main . [my table] begin select 1; end",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let a = |sql: &str| analyze_ddl(sql, &schema, &Extras::default()).analysis;
        // The table as the statement spells it, bare, quoted or qualified, is the table the DDL
        // declares, and its keys are that table's.
        for sql in [
            "select * from books where id = :id",
            "select * from BOOKS where ID = :id",
            "select * from \"books\" where \"ID\" = :id",
            "select * from [BOOKS] b where B.Id = :id",
            "select b.* from main.bOOKS b join Books c on c.ID = b.id where C.id = :id",
            "select title from books where isbn = :i",
            "update BOOKS set TITLE = :t where ID = :id returning Title",
            "delete from books where Isbn = :i returning id",
            "select v from WR where k = :k",
            "select v from Rt where _ROWID_ = :r",
        ] {
            let got = a(sql);
            assert!(got.diagnostics.is_empty(), "{sql}: {:?}", got.diagnostics);
            assert_eq!(got.cardinality, "maybeOne", "{sql}: {:?}", got.cardinality_reason);
        }
        // A declared column named rowid in any case hides the rowid, and a table without a rowid
        // has none to name.
        assert_eq!(a("select v from RT where ROWID = :r").cardinality, "many");
        assert_eq!(a("select v from wr where rowid = 1").diagnostics[0].message, "no such column: rowid");
        // A target column in another case is the declared column: its type, and its NOT NULL.
        let insert = a("insert into BOOKS (TITLE, NOTE) values (:t, :n)");
        assert_eq!((param(&insert, "t"), param(&insert, "n")), ("text".into(), "text".into()));
        assert_eq!(insert.params.iter().map(|p| p.nullable).collect::<Vec<_>>(), [false, true]);
        assert_eq!(col(&a("select TITLE, Note from books"), "Title"), ("text".into(), false));
        // The rowid by any of its names, in any case, is the INTEGER PRIMARY KEY column.
        assert_eq!(param(&a("insert into books (ROWID, title) values (:r, :t)"), "r"), "integer");
        assert_eq!(param(&a("insert into rt (_rowid_, v) values (:r, :v)"), "r"), "unknown");
        // SQLite folds ASCII alone: `été` is another name than `Été`, and no table.
        assert_eq!(a("select v from \"\u{c9}t\u{e9}\" where id = :id").cardinality, "maybeOne");
        assert_eq!(a("select v from \"\u{e9}t\u{e9}\" where id = :id").diagnostics[0].message, "no such table: \u{e9}t\u{e9}");
        // A table-valued function in any case is the function.
        assert_eq!(param(&a("select value from JSON_EACH(:ids)"), "ids"), "text");
        // A trigger's table is read as SQLite reads it, quoted and in another case, and so is a
        // trigger whose name or column list holds a keyword.
        assert_eq!(a("insert into books (title) values (:t) returning id").cardinality, "maybeOne");
        assert_eq!(a("insert into \"My Table\" (v) values (:v) returning id").cardinality, "maybeOne");
        assert_eq!(trigger_header(&schema[5]), Some(("before".into(), "insert".into(), "books".into())));
        assert_eq!(trigger_header(&schema[7]), Some(("after".into(), "update".into(), "my table".into())));
        assert_eq!(trigger_header("CREATE TRIGGER main.\"before\" /* on x */ INSTEAD OF DELETE ON v -- on y\n BEGIN SELECT 1; END"), Some(("instead".into(), "delete".into(), "v".into())));
        assert_eq!(trigger_header("create table trigger_log (\"on\" text)"), None);
    }

    #[test]
    fn every_row_of_a_values_list_types_its_parameters() {
        let a = analysis("insert into users (id, name, email) values (:a, :b, :e), (:c, :d, :e2), (:f, upper(:g), 'x') returning id");
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        let got: Vec<(String, String, bool)> = a.params.iter().map(|p| (p.name.clone(), scalar_name(&p.ty).unwrap_or("unknown").to_string(), p.nullable)).collect();
        let want = [("a", "integer", false), ("b", "text", false), ("e", "text", true), ("c", "integer", false), ("d", "text", false), ("e2", "text", true), ("f", "integer", false), ("g", "text", false)];
        assert_eq!(got, want.map(|(n, t, null)| (n.to_string(), t.to_string(), null)));
        assert_eq!((a.cardinality.as_str(), a.cardinality_reason.as_deref()), ("many", Some("a VALUES list of 3 rows returns 3 rows")));
        // A parameter a row does not write whole cannot be null. Each parameter appears once, so
        // one cannot be written to a nullable column in one row and a NOT NULL one in another.
        let a = analysis("insert into users (name, email) values ('a', :x), (:w, 'b'), ('c', lower(:y))");
        assert_eq!(a.params.iter().map(|p| p.nullable).collect::<Vec<_>>(), [true, false, false]);
        assert_eq!(a.cardinality, "none");
        let a = analysis("insert into users (name, email) values ('a', :x), (:x, 'b')");
        assert_eq!(a.diagnostics.iter().map(|d| d.code).collect::<Vec<_>>(), [3]);
        // Without a column list the rows fill the columns in order; under a WITH too.
        let a = analysis("with w as (select 1) insert into posts values (:id, :u, :t, :b), (:id2, :u2, :t2, :b2)");
        assert_eq!(a.params.iter().map(|p| (scalar_name(&p.ty).unwrap_or("unknown"), p.nullable)).collect::<Vec<_>>(), [("integer", false), ("integer", false), ("text", false), ("text", true)].repeat(2));
        // A list of any length: SQLite keeps a row per arm, and the tree nests none of them.
        let rows: Vec<String> = (0..3000).map(|i| format!("(:n{i}, :e{i}, {i})")).collect();
        let a = analysis(&format!("insert into users (name, email, score) values {}", rows.join(", ")));
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!((a.params.len(), param(&a, "n2999"), param(&a, "e2999")), (6000, "text".into(), "text".into()));
        // A VALUES list on its own: a row per row, a column of the type its rows share, NULL where
        // a row has one, and a parameter typed by the rows beside it.
        let a = analysis("values (1, 'a', :p), (2, null, 3), (:q, 'c', 4.5)");
        assert_eq!((col(&a, "column1"), col(&a, "column2")), (("integer".into(), false), ("text".into(), true)));
        assert_eq!((param(&a, "p"), param(&a, "q"), a.cardinality.as_str()), ("real".into(), "integer".into(), "many"));
        assert_eq!(analysis("values (1, 'a')").cardinality, "one");
        assert_eq!(col(&analysis("values (1), ('a')"), "column1").0, "unknown");
        assert_eq!(analysis("select v.column1 from (values (1), (2)) v").cardinality, "many");
        assert_eq!(analysis("select 1 union all values (2), (3)").cardinality, "many");
        assert_eq!(col(&analysis("select (values (1), (2)) as x"), "x"), ("integer".into(), true));
    }

    #[test]
    fn a_collate_keeps_its_operands_context() {
        let count = |sql: &str| {
            let mut schema = ddl();
            schema.push("create table c (id integer primary key, b text unique, n text unique collate nocase)".into());
            let a = analyze_ddl(sql, &schema, &Extras::default()).analysis;
            assert!(a.diagnostics.is_empty(), "{sql}: {:?}", a.diagnostics);
            (param(&a, "v"), a.cardinality)
        };
        assert_eq!(count("select id from users where name = :v collate nocase"), ("text".into(), "many".into()));
        assert_eq!(count("select id from users where :v collate nocase = name"), ("text".into(), "many".into()));
        assert_eq!(count("select id from users where name collate nocase = :v"), ("text".into(), "many".into()));
        assert_eq!(count("select id from users where id = :v collate binary"), ("integer".into(), "many".into()));
        assert_eq!(count("select id from users where name in (:v collate nocase, 'a')"), ("text".into(), "many".into()));
        // The type is the column's; the key is not the comparison's: a BINARY unique index tells
        // 'a' from 'A', and NOCASE meets both, on either side of the comparison.
        for sql in ["select id from c where b = :v collate nocase", "select id from c where :v collate nocase = b", "select id from c where b collate nocase = :v", "delete from c where b = :v collate nocase returning id"] {
            assert_eq!(count(sql), ("text".into(), "many".into()), "{sql}");
        }
        // A key under the comparison's own collation still bounds it, with no COLLATE written.
        assert_eq!(count("select id from c where n = :v"), ("text".into(), "maybeOne".into()));
        assert_eq!(count("select id from c where b = :v"), ("text".into(), "maybeOne".into()));
    }

    #[test]
    fn the_select_that_feeds_an_insert_types_its_parameters() {
        let schema = vec!["create table bar (name text not null, ready bool not null, n integer)".to_string(), "create table foo (name text not null, meta text, k integer)".to_string(), "create table foo2 (name text not null, meta text, k integer)".to_string()];
        let a = |sql: &str| {
            let a = analyze_ddl(sql, &schema, &Extras::default()).analysis;
            assert!(a.diagnostics.is_empty(), "{sql}: {:?}", a.diagnostics);
            a
        };
        let params = |a: &Analysis| a.params.iter().map(|p| (scalar_name(&p.ty).unwrap_or("unknown").to_string(), p.nullable)).collect::<Vec<_>>();
        let typed = |types: &[&str]| types.iter().map(|t| (t.to_string(), false)).collect::<Vec<_>>();
        // A select-list parameter takes its target column's type; one the WHERE compares, the
        // column's: `bool` is a declared type of NUMERIC affinity, a number or a text.
        let got = a("insert into foo (name, meta) select name, ? from bar where ready = ?");
        assert_eq!((got.kind.as_str(), got.cardinality.as_str()), ("insert", "none"));
        assert_eq!(params(&got), typed(&["text", "numeric"]));
        assert_eq!(params(&a("select name from bar where ready = ?")), typed(&["numeric"]));
        // Without a column list, through an expression, in every arm of a compound, and under a
        // LIMIT; a SELECT's column is not a value written whole, so it takes no NULL.
        assert_eq!(params(&a("insert into foo select :n, :m, :k from bar")), typed(&["text", "text", "integer"]));
        assert_eq!(params(&a("insert into foo (k, name) select n + :d, upper(:u) from bar limit :l")), typed(&["integer", "text", "integer"]));
        assert_eq!(params(&a("insert into foo (name, k) select name, :a from bar union all select :b, n from bar")), typed(&["integer", "text"]));
        let returning = a("insert into foo (name, meta) select name, :m from bar where n > :n returning name, meta");
        assert_eq!((params(&returning), returning.cardinality.as_str()), (typed(&["text", "integer"]), "many"));
        // A subquery elsewhere in the statement is not the INSERT's source: one in RETURNING
        // stands in no typed place, and one in a VALUES row is the value written to its own
        // column (k, not the first target column name), as any scalar subquery takes the type of
        // the place it stands in.
        assert_eq!(params(&a("insert into foo (name, meta) select name, 'x' from bar returning (select :p) as p")), typed(&["unknown"]));
        assert_eq!(params(&a("insert into foo (name, k) values ('a', (select :p))")), typed(&["integer"]));
        // A whole-table copy, which SQLite runs without a SELECT, is still an INSERT.
        let copy = a("insert into foo2 select * from foo");
        assert_eq!((copy.kind.as_str(), copy.cardinality.as_str()), ("insert", "none"));
    }

    #[test]
    fn the_hosts_sqlite_decides_which_functions_exist() {
        // The embedded SQLite has the math functions and `soundex`; a host may have neither.
        let sql = "select id, ACOS(score) as a from users where name = :n";
        let a = on_host(sql, &ddl(), &[], Vec::new());
        assert_eq!(a.diagnostics.len(), 1, "{:?}", a.diagnostics);
        assert_eq!(a.diagnostics[0].message, "no such function: ACOS on this database's SQLite");
        assert_eq!((a.diagnostics[0].start, a.diagnostics[0].length, a.diagnostics[0].code), (sql.find("ACOS").unwrap(), 4, 6));
        assert!(a.columns.is_empty());
        let a = on_host(sql, &ddl(), &[("acos", 1, 's')], Vec::new());
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!(col(&a, "a"), ("real".into(), true));
        let a = on_host("select soundex(name) as s from users where soundex(:x) = 'A000'", &ddl(), &[("SOUNDEX", 1, 's')], Vec::new());
        assert_eq!((col(&a, "s"), param(&a, "x")), (("text".into(), false), "text".into()));
        assert_eq!(on_host("select soundex(name) as s from users", &ddl(), &[], Vec::new()).diagnostics[0].message, "no such function: soundex on this database's SQLite");
        // A host that cannot list its functions keeps every function the embedded SQLite has.
        assert_eq!(col(&analysis("select acos(score) as a, soundex(name) as s from users"), "a"), ("real".into(), true));
        // What every SQLite has stays: operators that are functions underneath, keywords, `*`.
        let a = on_host("select count(*) as n, max(name) as m, coalesce(email, 'x') as e, current_timestamp as t, email ->> '$.a' as j, row_number() over (order by id) as r from users where name like :p and email glob 'a*' and iif(id > 1, true, false)", &ddl(), &[], Vec::new());
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        // A call inside a window definition, a subquery, a WITH query and an ORDER BY is a call.
        for sql in [
            "select row_number() over (order by sqrt(score)) as r from users",
            "select id from users where id in (select id from users where sqrt(score) > 1)",
            "with w as (select sqrt(score) as s from users) select s from w",
            "select id from users order by sqrt(score)",
            "update users set score = sqrt(score) where id = :id",
            "insert into users (name, score) values (:n, sqrt(:s))",
            "delete from users where id = :id returning sqrt(score) as s",
        ] {
            let a = on_host(sql, &ddl(), &[], Vec::new());
            assert_eq!(a.diagnostics.first().map(|d| (d.message.as_str(), d.start)), Some(("no such function: sqrt on this database's SQLite", sql.find("sqrt").unwrap())), "{sql}");
        }
        // A view that calls one fails every statement that reads it, with no place in the
        // statement to point at; the replay itself keeps the view.
        let mut schema = ddl();
        schema.push("create view roots as select id, sqrt(score) as root from users".into());
        let sql = "select root from roots";
        let a = on_host(sql, &schema, &[], Vec::new());
        assert_eq!(a.diagnostics.len(), 1, "{:?}", a.diagnostics);
        assert_eq!((a.diagnostics[0].message.as_str(), a.diagnostics[0].start, a.diagnostics[0].length), ("no such function: sqrt on this database's SQLite (the view roots calls it)", 0, sql.len()));
        assert_eq!(col(&on_host(sql, &schema, &[("sqrt", 1, 's')], Vec::new()), "root"), ("real".into(), true));
        assert!(on_host("select id from users", &schema, &[], Vec::new()).diagnostics.is_empty());
        // A function the host has and the embedded SQLite lacks prepares, with no type; an
        // aggregate is one row, and a window function takes an OVER.
        let extra = [("host_only", 2, 's'), ("host_only", 3, 's'), ("host_agg", 1, 'a'), ("host_win", -1, 'w')];
        let a = on_host("select host_only(name, 1) as two, host_only(name, 1, 2) as three from users where id = :id", &ddl(), &extra, Vec::new());
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!((col(&a, "two"), col(&a, "three"), a.cardinality.as_str()), (("unknown".into(), true), ("unknown".into(), true), "maybeOne"));
        // It may change from row to row, so it pins no key.
        assert_eq!(on_host("select name from users where id = host_only(1, 2)", &ddl(), &extra, Vec::new()).cardinality, "many");
        let a = on_host("select host_agg(score) as total from users", &ddl(), &extra, Vec::new());
        assert_eq!((col(&a, "total"), a.cardinality.as_str()), (("unknown".into(), true), "one"));
        let a = on_host("select host_win(score, 1) over (partition by name) as w, host_win(id) as plain from users", &ddl(), &extra, Vec::new());
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!(on_host("select host_only(name) from users", &ddl(), &extra, Vec::new()).diagnostics[0].message, "wrong number of arguments to function host_only()");
        // A function the host registers on its connection is one it has, listed or not.
        let html = vec![TotalFunction { name: "reylo_html".into(), arity: 1, returns: oid::TEXT }];
        assert_eq!(col(&on_host("select reylo_html(name) as h from users", &ddl(), &[], html), "h"), ("text".into(), false));
        // No list at all is no list: every SQLite has functions.
        assert_eq!(host_functions(&[]), None);
        assert_eq!(host_functions(&[vec![serde_json::json!("abs"), serde_json::json!("1"), serde_json::json!("s")]]), Some(vec![HostFunction { name: "abs".into(), arity: 1, kind: 's' }]));
    }

    #[test]
    fn the_replay_asks_the_host_for_its_functions_with_the_schema() {
        let sql = "select acos(score) as a from users";
        let lexed = lex::lex(sql, Dialect::Sqlite).unwrap();
        let schema = crate::api::Answer::Rows { rows: ddl().iter().map(|d| vec![serde_json::Value::String(d.clone())]).collect() };
        let ask = |answers: &HashMap<String, crate::api::Answer>| {
            let host = Host::new(answers);
            let analysis = analyze(sql, &lexed, &host);
            let needs: Vec<String> = host
                .needs()
                .iter()
                .map(|n| match n {
                    crate::api::Need::Query { sql, scope, .. } => format!("{sql} {scope:?}"),
                    crate::api::Need::Describe { sql, .. } => format!("describe {sql}"),
                })
                .collect();
            (analysis, needs)
        };
        // One round asks both, for the session.
        let mut answers = HashMap::new();
        let (analysis, needs) = ask(&answers);
        assert!(analysis.is_none());
        assert_eq!(needs, [format!("{DDL_QUERY} Session"), format!("{FUNCTIONS_QUERY} Session")]);
        answers.insert(format!("query:{DDL_QUERY}|[]"), schema);
        assert_eq!(ask(&answers).1, [format!("{FUNCTIONS_QUERY} Session")]);
        // The host's list decides.
        answers.insert(format!("query:{FUNCTIONS_QUERY}|[]"), functions_answer(&[]));
        let (analysis, needs) = ask(&answers);
        assert!(needs.is_empty());
        assert_eq!(analysis.unwrap().diagnostics[0].message, "no such function: acos on this database's SQLite");
        answers.insert(format!("query:{FUNCTIONS_QUERY}|[]"), functions_answer(&[("acos", 1, 's')]));
        assert!(ask(&answers).0.unwrap().diagnostics.is_empty());
        // A host whose SQLite refuses the question, or answers nothing, keeps the embedded one's.
        answers.insert(format!("query:{FUNCTIONS_QUERY}|[]"), crate::api::Answer::Error { error: crate::api::DbError { message: "not authorized".into(), code: None, position: None } });
        assert_eq!(col(&ask(&answers).0.unwrap(), "a"), ("real".into(), true));
        answers.insert(format!("query:{FUNCTIONS_QUERY}|[]"), crate::api::Answer::Rows { rows: Vec::new() });
        assert_eq!(col(&ask(&answers).0.unwrap(), "a"), ("real".into(), true));
    }

    #[test]
    fn a_table_in_an_attached_database_is_named_as_one() {
        let message = |schema: &str| format!("no such table: {schema}.users: the database has no schema `{schema}`. A database the application attaches at run time (ATTACH ... AS {schema}) is not in the file the type check reads, so a table in it cannot be typed");
        for (sql, reference) in [
            ("select id, name from baz.users where id = :id", "baz.users"),
            ("select u.id from \"baz\".[users] u", "\"baz\".[users]"),
            ("select 1 from users join /* x */ baz . users b on b.id = users.id", "baz . users"),
            ("insert into baz.users (id) values (:id)", "baz.users"),
            ("update baz.users set name = :n where id = :id", "baz.users"),
            ("delete from baz.users where id = :id", "baz.users"),
        ] {
            let a = analysis(sql);
            assert_eq!(a.diagnostics.len(), 1, "{sql}: {:?}", a.diagnostics);
            assert_eq!(a.diagnostics[0].message, message("baz"), "{sql}");
            assert_eq!((a.diagnostics[0].start, a.diagnostics[0].length), (sql.find(reference).unwrap(), reference.len()), "{sql}");
        }
        // The position survives the placeholder rewrite.
        let sql = "select :auth.user_id as u, id from baz.users";
        assert_eq!(analysis(sql).diagnostics[0].start, sql.find("baz").unwrap());
        // A schema the database has, and a name that only holds a dot, are SQLite's own message.
        assert_eq!(analysis("select id from main.nope").diagnostics[0].message, "no such table: main.nope");
        assert_eq!(analysis("select id from temp.nope").diagnostics[0].message, "no such table: temp.nope");
        assert_eq!(analysis("select id from \"baz.users\"").diagnostics[0].message, "no such table: baz.users");
        assert_eq!(analysis("select id from nope").diagnostics[0].message, "no such table: nope");
        assert!(analysis("select main.users.id from main.users").diagnostics.is_empty());
    }

    #[test]
    fn sqlites_own_tables_replay_without_a_note() {
        // `sqlite_schema` lists SQLite's own tables beside the application's: the sequence table
        // an AUTOINCREMENT makes, the statistics ANALYZE writes, a virtual table's shadow tables.
        let schema: Vec<String> = [
            "CREATE TABLE t (id integer primary key autoincrement, v text)",
            "CREATE TABLE sqlite_sequence(name,seq)",
            "CREATE TABLE sqlite_stat1(tbl,idx,stat)",
            "CREATE VIRTUAL TABLE ft USING fts5(body)",
            "CREATE TABLE 'ft_data'(id INTEGER PRIMARY KEY, block BLOB)",
            "CREATE TABLE 'ft_idx'(segid, term, pgno, PRIMARY KEY(segid, term)) WITHOUT ROWID",
            "CREATE TABLE 'ft_content'(id INTEGER PRIMARY KEY, c0)",
            "CREATE TABLE 'ft_docsize'(id INTEGER PRIMARY KEY, sz BLOB)",
            "CREATE TABLE 'ft_config'(k PRIMARY KEY, v) WITHOUT ROWID",
            "CREATE VIRTUAL TABLE rt USING rtree(id, minx, maxx)",
            "CREATE TABLE \"rt_rowid\"(rowid INTEGER PRIMARY KEY,nodeno)",
            "CREATE TABLE \"rt_node\"(nodeno INTEGER PRIMARY KEY,data)",
            "CREATE TABLE \"rt_parent\"(nodeno INTEGER PRIMARY KEY,parentnode)",
            "CREATE VIRTUAL TABLE f4 USING fts4(body)",
            "CREATE TABLE 'f4_content'(docid INTEGER PRIMARY KEY, 'c0body')",
            "CREATE TABLE 'f4_segments'(blockid INTEGER PRIMARY KEY, block BLOB)",
            "CREATE TABLE 'f4_segdir'(level INTEGER,idx INTEGER,start_block INTEGER,leaves_end_block INTEGER,end_block INTEGER,root BLOB,PRIMARY KEY(level, idx))",
            "CREATE TABLE 'f4_docsize'(docid INTEGER PRIMARY KEY, size BLOB)",
            "CREATE TABLE 'f4_stat'(id INTEGER PRIMARY KEY, value BLOB)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let a = |sql: &str, schema: &[String]| analyze_ddl(sql, schema, &Extras::default()).analysis;
        // An error in the statement is the only thing reported.
        let failed = a("select nope from t", &schema);
        assert_eq!(failed.diagnostics.iter().map(|d| d.message.as_str()).collect::<Vec<_>>(), ["no such column: nope"]);
        for sql in [
            "select name, seq from sqlite_sequence where name = :n",
            "update sqlite_sequence set seq = :s where name = :n",
            "select tbl, idx, stat from sqlite_stat1",
            "select name, sql from sqlite_schema where type = 'table'",
            "select name from sqlite_master",
            "select rowid, body from ft where ft match :q",
            "select id from rt where minx > :x",
            "select docid from f4 where f4 match :q",
            "select block from ft_data where id = :id",
            "insert into t (v) values (:v) returning id",
        ] {
            let got = a(sql, &schema);
            assert!(got.diagnostics.is_empty(), "{sql}: {:?}", got.diagnostics);
        }
        assert_eq!(a("insert into t (v) values (:v) returning id", &schema).cardinality, "one");
        // The schema table itself stays read-only, as on the host.
        assert_eq!(a("delete from sqlite_schema", &schema).diagnostics[0].message, "table sqlite_master may not be modified");
        // A sequence table that outlived its AUTOINCREMENT table, before one made later.
        let later = vec!["CREATE TABLE sqlite_sequence(name,seq)".to_string(), "CREATE TABLE u (id integer primary key autoincrement, v text)".to_string()];
        assert!(a("select seq from sqlite_sequence", &later).diagnostics.is_empty());
        assert_eq!(a("insert into u (v) values (:v) returning id", &later).cardinality, "one");
        assert_eq!(a("select nope from u", &later).diagnostics.len(), 1);
        // A step that really fails is still named beside the statement's error.
        let broken = vec!["CREATE TABLE t (id integer primary key)".to_string(), "CREATE TABLE t (id integer primary key)".to_string(), "CREATE VIEW v AS SELECT".to_string()];
        let notes: Vec<String> = a("select nope from t", &broken).diagnostics.iter().map(|d| d.message.clone()).collect();
        assert_eq!(notes, ["no such column: nope", "The schema replay skipped statement 2: table t already exists", "The schema replay skipped statement 3: incomplete input"]);
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
        Extras { total: vec![TotalFunction { name: "reylo_html".into(), arity: 1, returns: oid::TEXT }], host: None }
    }

    #[test]
    fn a_host_function_prepares_and_is_typed_from_its_argument() {
        let r = analyze_ddl("select reylo_html(name) as h, reylo_html(email) as e, upper(reylo_html(:q)) as q from users", &ddl(), &html());
        let a = &r.analysis;
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
        assert_eq!(col(a, "h"), ("text".into(), false));
        assert_eq!(col(a, "e"), ("text".into(), true));
        // The parameter is a non-null value, so the call over it is too.
        assert_eq!(col(a, "q"), ("text".into(), false));
        assert_eq!(r.declared, [None, None, None]);
        // The same call without the registration, and with the wrong argument count: SQLite's own
        // message, at the call.
        let sql = "select id, reylo_html(name) as h from users";
        for (extras, message) in [(Extras::default(), "no such function: reylo_html"), (Extras { total: vec![TotalFunction { name: "reylo_html".into(), arity: 2, returns: oid::TEXT }], host: None }, "wrong number of arguments to function reylo_html()")] {
            let a = analyze_ddl(sql, &ddl(), &extras).analysis;
            assert_eq!(a.diagnostics.len(), 1, "{:?}", a.diagnostics);
            assert_eq!(a.diagnostics[0].message, message);
            assert_eq!((a.diagnostics[0].start, a.diagnostics[0].code), (sql.find("reylo_html").unwrap(), 6));
            assert!(a.columns.is_empty());
        }
        // Any argument count, and a schema that calls the function replays.
        let any = Extras { total: vec![TotalFunction { name: "reylo_html".into(), arity: -1, returns: oid::TEXT }], host: None };
        let mut schema = ddl();
        schema.push("create view safe_users as select id, reylo_html(name) as name from users".into());
        let a = analyze_ddl("select name, reylo_html(name, name) as twice from safe_users", &schema, &any).analysis;
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

