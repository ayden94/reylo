//! Static cardinality: whether a statement returns exactly one row, proven from its shape.
//!
//! A middleware returns exactly one row, and the build refuses one it cannot prove does. An
//! endpoint's `returns:` is checked against the same reading, but only for a contradiction it
//! proves — `returns: one` over `LIMIT 0` — never for a shape it merely cannot see through,
//! because `WHERE id = :id` is unprovable and correct.
//!
//! The proof is small and fail-closed: anything it does not recognise is unproven, and
//! "unproven" never means "proven otherwise". The runtime still counts every middleware's rows,
//! because the database the request runs against is not the one the build read (a function can
//! be recreated as set-returning after the build, and a prepared statement re-plans onto it).
//!
//! A `SELECT` returns exactly one row when all of these hold:
//!
//! 1. It has the shape of one row, which is [`infer::single_row`]'s reading: no set operation, no
//!    `VALUES`, no `GROUP BY` (a row per group, and none over no rows), no `HAVING` (it can drop
//!    the one group), no `LIMIT` or `OFFSET` (`LIMIT 0` and `OFFSET 1` drop the row).
//! 2. It aggregates, or it has neither a `FROM` nor a `WHERE`. An aggregate with no `GROUP BY` is
//!    one group over any input, empty included (`SELECT count(*) FROM t WHERE false` and even
//!    `SELECT count(*) WHERE false` are one row on both dialects); whether a target aggregates is
//!    [`infer::aggregates`]'s reading. Without one, a `FROM` is a row per row it matches, and a
//!    `WHERE` with no `FROM` drops the one row when it is false (`SELECT 1 WHERE :x = 1`).
//! 3. Nothing at its own level returns a set (Postgres only; see below). `SELECT
//!    generate_series(1, 3)` is three rows, `SELECT unnest('{}'::int[])` none, and an aggregate
//!    beside one is as many rows as the set (`SELECT count(*), generate_series(1, 3) FROM t`).
//!
//! A `WITH` changes none of this: the main statement reads a CTE only through a `FROM` or a
//! subquery. A subquery answers one value to the level above it whatever it returns: `EXISTS`
//! is one boolean, and a scalar subquery with two rows is an error on Postgres and its first row
//! on SQLite, never two outer rows.
//!
//! # Set-returning calls on Postgres
//!
//! In a select list, Postgres multiplies a row only by evaluating a set-returning function or
//! operator, and the parser only produces one from a call or an operator spelled in the
//! statement: a cast function must not return a set (`CREATE CAST` refuses one), and `CASE`,
//! `COALESCE`, `IS TRUE`, `IN (…)`, `WHERE`, `HAVING`, `LIMIT` and a window definition refuse
//! one outright. But one nested anywhere else in an expression at this level still multiplies the
//! row — under a function (`abs(generate_series(-1, 1))`), a cast, `IS NOT NULL`, `IS DISTINCT
//! FROM`, `NULLIF`'s argument — and so does one in `ORDER BY` or `DISTINCT ON`, which the planner
//! evaluates with the select list. `CREATE OPERATOR` over a `SETOF` function makes an operator
//! that multiplies rows exactly as the function does.
//!
//! So the proof reads every function name and every operator name at this level — the select
//! list, `ORDER BY`, `DISTINCT ON`, window definitions, and each subquery's tested expression
//! (`x IN (SELECT …)`), but not a subquery's own statement — and looks each up in the catalog by
//! name alone, across every overload and schema ([`Catalog`]). The resolver can only pick an
//! overload of a name it was given, so reading all of them is a superset of whatever it picks. A
//! name the catalog does not have is unproven too.
//!
//! SQLite has no set-returning function a select list may call — `json_each` and
//! `generate_series` are table-valued, live in `FROM`, and are "no such function" in a select
//! list — and no way to define an operator, so there the rule is empty.
//!
//! SQLite statements are read with libpg_query, as `infer` and `jsonshape` read them, after their
//! `?n` placeholders become `$n`. Clause structure is the SQL both dialects share, and SQLite's
//! own syntax — `LIMIT 5, 0`, `[bracketed]` names — does not parse there, which proves nothing.

use crate::dialect::Dialect;
use crate::infer::{self, Trust};
use pg_query::protobuf::{self, node::Node as NodeEnum, SelectStmt};
use serde_json::Value;
use std::collections::HashSet;

/// The shapes the build proves return exactly one row, for the diagnostic that refuses a
/// middleware it cannot prove.
pub const PROVABLE: &str = "a SELECT with no FROM and no WHERE, such as `SELECT EXISTS (SELECT 1 FROM org_members WHERE org_id = :org_id AND user_id = :auth.user_id) AS authorized` or `SELECT (SELECT role FROM org_members WHERE org_id = :org_id AND user_id = :auth.user_id) AS role`; or a SELECT whose select list calls an aggregate (count, sum, avg, min, max) outside a subquery and without OVER, such as `SELECT count(*) > 0 AS authorized FROM org_members WHERE org_id = :org_id AND user_id = :auth.user_id`. Either may start with a WITH. Neither may have GROUP BY, HAVING, LIMIT, OFFSET, UNION, INTERSECT, EXCEPT, or VALUES, or call a set-returning function such as generate_series or unnest outside a subquery";

/// The catalog facts the proof needs and the parse tree cannot carry, all by name. The host
/// answers the three queries below and the orchestration fills this in.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    /// Function names with at least one entry in `pg_proc`.
    pub functions: HashSet<String>,
    /// Function names with at least one set-returning overload (`proretset`), in any schema.
    pub set_returning: HashSet<String>,
    /// Operator names some set-returning function implements, in any schema.
    pub set_returning_operators: HashSet<String>,
    /// Per relation, the column lists of its unique keys: every unique, valid, non-partial index
    /// over plain columns. This is the fact [`at_most_one`] rests on, and it is a fact about the
    /// database rather than about the statement — adding an index makes a read provably single
    /// and dropping one makes it plural again, which is the inference doing its job.
    pub unique_keys: std::collections::HashMap<String, Vec<Vec<String>>>,
}

/// `$1` is `text[]` of relation names: one row per unique index, `(relname, key columns)`. An
/// index over an expression loses a column to the join and the HAVING then drops it, because the
/// proof can only match a key it can name. A partial one restricts which rows it covers, so it
/// proves nothing about a row the statement's own WHERE selects.
pub const UNIQUE_KEYS_QUERY: &str = "SELECT c.relname::text, array_agg(a.attname::text ORDER BY k.ord) \
    FROM pg_index i \
    JOIN pg_class c ON c.oid = i.indrelid \
    JOIN unnest(i.indkey::int2[]) WITH ORDINALITY AS k(attnum, ord) ON k.ord <= i.indnkeyatts \
    JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = k.attnum AND NOT a.attisdropped \
    WHERE i.indisunique AND i.indpred IS NULL AND i.indisvalid \
      AND c.relname = ANY($1::text[]) AND pg_catalog.pg_table_is_visible(c.oid) \
    GROUP BY c.relname, i.indexrelid, i.indnkeyatts \
    HAVING count(*) = i.indnkeyatts";

/// `$1` is `text[]` of function names: `(proname, has a set-returning overload)`.
pub const FUNCTIONS_QUERY: &str =
    "SELECT proname::text, bool_or(proretset) FROM pg_proc WHERE proname::text = ANY($1::text[]) GROUP BY proname";

/// Every operator some set-returning function implements.
pub const SET_RETURNING_OPERATORS_QUERY: &str =
    "SELECT DISTINCT o.oprname::text FROM pg_operator o JOIN pg_proc p ON p.oid = o.oprcode WHERE p.proretset";

/// Prove that one rewritten statement (placeholders already `$n` or `?n`) returns exactly one
/// row, or say why the build cannot, worded to finish a sentence: "a WHERE with no FROM can
/// remove the row". `trust` is the describer's ([`crate::describe::Describer::trust`]); `catalog`
/// is read on Postgres only.
pub fn exactly_one(sql: &str, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    let s = one_select(sql, dialect)?;
    exactly_one_select(&s, dialect, trust, catalog)
}

fn exactly_one_select(s: &SelectStmt, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    if !infer::single_row(s) {
        return Err(not_one_row_shaped(s).to_string());
    }
    if s.into_clause.is_some() {
        return Err("SELECT INTO creates a table and returns no rows".into());
    }
    if dialect == Dialect::Postgres {
        let at_this_level = s.target_list.iter().chain(&s.sort_clause).chain(&s.distinct_clause).chain(&s.window_clause);
        if let Some(why) = calls_at_this_level(at_this_level).set_returning(catalog) {
            return Err(why);
        }
    }
    let aggregates = s.target_list.iter().any(|target| match target.node.as_ref() {
        Some(NodeEnum::ResTarget(rt)) => rt.val.as_deref().is_some_and(|val| infer::aggregates(val, dialect, trust)),
        _ => false,
    });
    if aggregates {
        return Ok(());
    }
    if !s.from_clause.is_empty() {
        return Err("a FROM returns a row per row it matches, and only an aggregate with no GROUP BY collapses them to one".into());
    }
    if s.where_clause.is_some() {
        return Err("a WHERE with no FROM can remove the row".into());
    }
    Ok(())
}

/// Prove that one rewritten statement returns **at most** one row — no row, or one — or say why
/// the build cannot, worded to finish a sentence the same way [`exactly_one`]'s does.
///
/// This is the weaker reading, and it is the one a nested `SUBQ` takes its shape from: at most one
/// row is an object or null, and anything else is a list. Three things prove it, in the order the
/// build tries them:
///
/// 1. A constant `LIMIT 0` or `LIMIT 1`, which caps the count whatever the rest of the statement
///    does — a set operation and a set-returning call included, because the limit applies last.
/// 2. Exactly one row, which is at most one row. That covers `SELECT count(*) …` and the shapes
///    with no `FROM` at all.
/// 3. A unique key of the one relation the statement reads, every column of it equated to
///    something that is not a column of that relation. Two rows cannot share a unique key, and an
///    equality never matches NULL, so at most one row survives — which holds whether or not the
///    key's columns are `NOT NULL`, since a unique index only lets NULLs repeat.
///
/// The third is why an index changes an inferred type. The proof reads the index that exists at
/// build time; `anti.lock` and the boot re-describe turn a later `CREATE`/`DROP INDEX` into a
/// contract mismatch the deploy reports, rather than a shape that quietly drifts.
pub fn at_most_one(sql: &str, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    let s = one_select(sql, dialect)?;
    if let Some(n) = const_limit(&s) {
        if n <= 1 {
            return Ok(());
        }
    }
    if exactly_one_select(&s, dialect, trust, catalog).is_ok() {
        return Ok(());
    }
    if s.limit_count.is_some() {
        // Anything still here is a LIMIT above one, or one written as an expression the build
        // cannot read. An OFFSET alone only ever removes rows, so it is left to the key proof.
        return Err("a LIMIT of more than one row, or one the build cannot read, bounds nothing".into());
    }
    if let Some(why) = not_key_shaped(&s) {
        return Err(why.to_string());
    }
    if dialect == Dialect::Postgres {
        let at_this_level = s.target_list.iter().chain(&s.sort_clause).chain(&s.distinct_clause).chain(&s.window_clause);
        if let Some(why) = calls_at_this_level(at_this_level).set_returning(catalog) {
            return Err(why);
        }
    }
    one_row_by_key(&s, catalog)
}

/// Which of the key proof's structural conditions a statement breaks, if it breaks one. A shape
/// that returns a row per group or per arm is not a shape a unique key speaks for.
fn not_key_shaped(s: &SelectStmt) -> Option<&'static str> {
    if s.larg.is_some() || s.rarg.is_some() {
        Some("a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows")
    } else if !s.values_lists.is_empty() {
        Some("a VALUES list returns a row per row it lists")
    } else if !s.group_clause.is_empty() {
        Some("GROUP BY returns one row per group")
    } else if s.having_clause.is_some() {
        Some("HAVING leaves a row per group")
    } else if s.into_clause.is_some() {
        Some("SELECT INTO creates a table and returns no rows")
    } else {
        None
    }
}

/// The unique-key proof: one relation, and a unique key of it every column of which the `WHERE`
/// equates to something outside the relation.
fn one_row_by_key(s: &SelectStmt, catalog: &Catalog) -> Result<(), String> {
    let [only] = s.from_clause.as_slice() else {
        return Err(match s.from_clause.len() {
            0 => "a SELECT with no FROM and a WHERE can return no row or one, and the engine proves that only through a LIMIT".into(),
            _ => "a join returns a row per pair it matches, and the engine proves a single row only for one relation".into(),
        });
    };
    let Some(NodeEnum::RangeVar(rv)) = only.node.as_ref() else {
        return Err("only a plain table or view in the FROM is matched against a unique key".into());
    };
    key_bounds(rv, s.with_clause.as_ref(), s.where_clause.as_deref(), catalog)
}

/// Whether `where_clause` pins one row of `rv` by a unique key.
fn key_bounds(
    rv: &protobuf::RangeVar,
    with: Option<&protobuf::WithClause>,
    where_clause: Option<&protobuf::Node>,
    catalog: &Catalog,
) -> Result<(), String> {
    let relation = rv.relname.clone();
    // A CTE of the same name shadows the table, and a CTE has no index at all.
    if let Some(with) = with {
        if with.ctes.iter().any(|c| cte_name(c) == Some(&relation)) {
            return Err(format!("`{relation}` here is the WITH query of that name, which has no unique index"));
        }
    }
    let Some(keys) = catalog.unique_keys.get(&relation).filter(|k| !k.is_empty()) else {
        return Err(format!("`{relation}` has no unique index the engine can read, so nothing bounds the rows a WHERE matches"));
    };
    let alias = rv.alias.as_ref().map(|a| a.aliasname.as_str()).unwrap_or(&relation);
    let equated = match where_clause {
        Some(w) => equated_columns(w, alias),
        None => HashSet::new(),
    };
    match keys.iter().find(|key| key.iter().all(|c| equated.contains(c))) {
        Some(_) => Ok(()),
        None => {
            let shown: Vec<String> = keys.iter().map(|k| format!("({})", k.join(", "))).collect();
            Err(format!("the WHERE does not equate every column of a unique key of `{relation}` — it has {}", shown.join(", ")))
        }
    }
}

/// How many rows a data-modifying statement's `RETURNING` produces, from its shape: an `INSERT`
/// of one `VALUES` row is one row (or none under `ON CONFLICT DO NOTHING`); an `UPDATE` or
/// `DELETE` whose `WHERE` pins a unique key of its one target relation is at most one; anything
/// else is any number. `None` when the statement is not one of these.
pub fn dml(sql: &str, catalog: &Catalog) -> Option<Result<Cardinality, String>> {
    let parsed = pg_query::parse(sql).ok()?;
    let mut stmts = parsed.protobuf.stmts;
    if stmts.len() != 1 {
        return None;
    }
    Some(match stmts.pop().and_then(|s| s.stmt).and_then(|n| n.node)? {
        NodeEnum::InsertStmt(i) => {
            let rows = match i.select_stmt.as_deref().and_then(|n| n.node.as_ref()) {
                Some(NodeEnum::SelectStmt(s)) if s.values_lists.len() == 1 && s.from_clause.is_empty() => 1,
                Some(NodeEnum::SelectStmt(s)) if !s.values_lists.is_empty() => s.values_lists.len(),
                // `INSERT … DEFAULT VALUES` has no select at all.
                None => 1,
                _ => return Some(Err("an INSERT … SELECT returns a row per row the SELECT produces".into())),
            };
            if rows != 1 {
                return Some(Err("a VALUES list returns a row per row it lists".into()));
            }
            match i.on_conflict_clause.as_ref().map(|c| c.action) {
                Some(a) if a == protobuf::OnConflictAction::OnconflictNothing as i32 => Ok(Cardinality::AtMostOne),
                _ => Ok(Cardinality::One),
            }
        }
        NodeEnum::UpdateStmt(u) => {
            if !u.from_clause.is_empty() {
                return Some(Err("an UPDATE … FROM returns a row per pair it matches".into()));
            }
            let rv = u.relation.as_ref()?;
            key_bounds(rv, u.with_clause.as_ref(), u.where_clause.as_deref(), catalog).map(|()| Cardinality::AtMostOne)
        }
        NodeEnum::DeleteStmt(d) => {
            if !d.using_clause.is_empty() {
                return Some(Err("a DELETE … USING returns a row per pair it matches".into()));
            }
            let rv = d.relation.as_ref()?;
            key_bounds(rv, d.with_clause.as_ref(), d.where_clause.as_deref(), catalog).map(|()| Cardinality::AtMostOne)
        }
        _ => return None,
    })
}

/// What the proofs settle on, for one statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    One,
    AtMostOne,
}

fn cte_name(cte: &protobuf::Node) -> Option<&String> {
    match cte.node.as_ref() {
        Some(NodeEnum::CommonTableExpr(c)) => Some(&c.ctename),
        _ => None,
    }
}

/// The columns of `alias` the `WHERE` equates to a value from outside it, reading the top-level
/// `AND`s only: an `OR` beside them widens the match, and anything else is not a restriction the
/// build reads. The other side must name no column of this relation, because `x = y` between two
/// of its own columns restricts nothing a unique key covers.
fn equated_columns(where_clause: &protobuf::Node, alias: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut stack = vec![where_clause];
    while let Some(node) = stack.pop() {
        match node.node.as_ref() {
            Some(NodeEnum::BoolExpr(b)) if b.boolop == protobuf::BoolExprType::AndExpr as i32 => {
                stack.extend(b.args.iter());
            }
            Some(NodeEnum::AExpr(e)) if is_equality(e) => {
                let (Some(l), Some(r)) = (e.lexpr.as_deref(), e.rexpr.as_deref()) else { continue };
                for (column, other) in [(l, r), (r, l)] {
                    if let Some(name) = own_column(column, alias) {
                        if !reads_relation(other, alias) {
                            out.insert(name);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// Whether this is the equality operator itself, rather than `= ANY (…)`, `IS NOT DISTINCT FROM`,
/// or an operator merely spelled with an equals sign.
fn is_equality(e: &protobuf::AExpr) -> bool {
    e.kind == protobuf::AExprKind::AexprOp as i32 && e.name.last().and_then(field_name) == Some("=")
}

/// The column name this expression is, when it names a column of `alias` — `alias.col`, or a bare
/// `col`, which SQL resolves to the innermost relation that has it, and a unique key's column is
/// one this relation has.
fn own_column(node: &protobuf::Node, alias: &str) -> Option<String> {
    let Some(NodeEnum::ColumnRef(c)) = node.node.as_ref() else { return None };
    let parts: Vec<&str> = c.fields.iter().filter_map(field_name).collect();
    match parts.as_slice() {
        [column] => Some((*column).to_string()),
        [qualifier, column] if *qualifier == alias => Some((*column).to_string()),
        _ => None,
    }
}

/// Whether this expression reads any column of `alias`, qualified or bare.
fn reads_relation(node: &protobuf::Node, alias: &str) -> bool {
    let Ok(tree) = serde_json::to_value(node) else { return true };
    fn walk(v: &Value, alias: &str) -> bool {
        match v {
            Value::Object(map) => map.iter().any(|(key, child)| {
                if key == "ColumnRef" {
                    let parts: Vec<String> = child
                        .get("fields")
                        .and_then(|f| f.as_array())
                        .map(|f| {
                            f.iter()
                                .filter_map(|n| n.get("node")?.get("String")?.get("sval")?.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    return match parts.as_slice() {
                        [_] => true,
                        [qualifier, _] => qualifier == alias,
                        _ => true,
                    };
                }
                walk(child, alias)
            }),
            Value::Array(items) => items.iter().any(|i| walk(i, alias)),
            _ => false,
        }
    }
    walk(&tree, alias)
}

fn field_name(node: &protobuf::Node) -> Option<&str> {
    match node.node.as_ref() {
        Some(NodeEnum::String(s)) => Some(s.sval.as_str()),
        _ => None,
    }
}

/// A constant `LIMIT n`, when the statement has one the build can read.
fn const_limit(s: &SelectStmt) -> Option<i64> {
    match s.limit_count.as_deref()?.node.as_ref()? {
        NodeEnum::AConst(c) if !c.isnull => match &c.val {
            Some(protobuf::a_const::Val::Ival(i)) => Some(i.ival as i64),
            _ => None,
        },
        _ => None,
    }
}

/// Whether one rewritten statement provably returns no row, with the reason: a constant `LIMIT 0`
/// (`FETCH FIRST 0 ROWS ONLY` is the same node). Nothing else is claimed, so an endpoint whose
/// `returns:` expects a row is refused only over a statement that can never have one.
pub fn never_a_row(sql: &str, dialect: Dialect) -> Option<&'static str> {
    let s = one_select(sql, dialect).ok()?;
    match s.limit_count.as_deref()?.node.as_ref()? {
        NodeEnum::AConst(c) if !c.isnull => match &c.val {
            Some(protobuf::a_const::Val::Ival(i)) if i.ival == 0 => Some("LIMIT 0 returns no rows"),
            _ => None,
        },
        _ => None,
    }
}

/// The one `SELECT` a statement is, parsed as the dialect's statements are read.
fn one_select(sql: &str, dialect: Dialect) -> Result<Box<SelectStmt>, String> {
    let text = match dialect {
        Dialect::Postgres => sql.to_string(),
        Dialect::Sqlite | Dialect::Mysql => infer::dollar_placeholders(sql),
    };
    let parsed = pg_query::parse(&text).map_err(|_| match dialect {
        Dialect::Postgres => "the statement does not parse".to_string(),
        _ => format!("the engine reads a {} statement's shape in the SQL it shares with Postgres, and this one uses syntax outside it", dialect.name()),
    })?;
    let mut stmts = parsed.protobuf.stmts;
    if stmts.len() != 1 {
        return Err("it is not exactly one statement".into());
    }
    match stmts.pop().and_then(|s| s.stmt).and_then(|n| n.node) {
        Some(NodeEnum::SelectStmt(s)) => Ok(s),
        _ => Err("only a SELECT is proven".into()),
    }
}

/// Which of `single_row`'s conditions a statement breaks, for the diagnostic. The decision is
/// `single_row`'s; this only names it.
fn not_one_row_shaped(s: &SelectStmt) -> &'static str {
    if s.larg.is_some() || s.rarg.is_some() {
        "a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows"
    } else if !s.values_lists.is_empty() {
        "a VALUES list returns a row per row it lists"
    } else if !s.group_clause.is_empty() {
        "GROUP BY returns one row per group, and none when nothing matches"
    } else if s.having_clause.is_some() {
        "HAVING can remove the row"
    } else {
        "a LIMIT or an OFFSET can remove the row (LIMIT 0, OFFSET 1)"
    }
}

/// The function and operator names of one query level.
#[derive(Debug, Default)]
struct Calls {
    functions: Vec<String>,
    operators: Vec<String>,
}

impl Calls {
    /// Why a call here may return a set, if one may.
    fn set_returning(&self, catalog: &Catalog) -> Option<String> {
        for name in &self.functions {
            if catalog.set_returning.contains(name) {
                return Some(format!("the set-returning function {name} returns a row per value, and none for an empty set"));
            }
            if !catalog.functions.contains(name) {
                return Some(format!("the function {name} is not in the catalog, so the build cannot rule out that it returns a set"));
            }
        }
        self.operators
            .iter()
            .find(|op| catalog.set_returning_operators.contains(op.as_str()))
            .map(|op| format!("the operator {op} is implemented by a set-returning function in this database"))
    }
}

fn calls_at_this_level<'a>(nodes: impl Iterator<Item = &'a protobuf::Node>) -> Calls {
    let mut calls = Calls::default();
    for node in nodes {
        if let Ok(tree) = serde_json::to_value(node) {
            walk(&tree, &mut calls);
        }
    }
    calls
}

fn last_name(names: Option<&Value>) -> Option<String> {
    names?.as_array()?.last()?.get("node")?.get("String")?.get("sval")?.as_str().map(String::from)
}

fn walk(v: &Value, calls: &mut Calls) {
    match v {
        Value::Object(map) => {
            for (key, child) in map {
                match key.as_str() {
                    // A subquery answers one value to this level; only the expression it tests
                    // (`x IN (SELECT …)`) and the operator it tests with are evaluated here.
                    "SubLink" => {
                        let tests = child.get("testexpr").is_some_and(|t| !t.is_null());
                        match last_name(child.get("oper_name")) {
                            Some(op) => calls.operators.push(op),
                            None if tests => calls.operators.push("=".into()),
                            None => {}
                        }
                        if let Some(test) = child.get("testexpr") {
                            walk(test, calls);
                        }
                    }
                    "FuncCall" => {
                        if let Some(name) = last_name(child.get("funcname")) {
                            calls.functions.push(name);
                        }
                        walk(child, calls);
                    }
                    "AExpr" => {
                        match last_name(child.get("name")) {
                            // `a BETWEEN x AND y` is spelled with the comparisons it expands to.
                            Some(op) if op.contains("BETWEEN") => calls.operators.extend(["<", ">", "<=", ">="].map(String::from)),
                            Some(op) => calls.operators.push(op),
                            None => {}
                        }
                        walk(child, calls);
                    }
                    _ => walk(child, calls),
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|i| walk(i, calls)),
        _ => {}
    }
}

