//! The MySQL analysis. The server's prepare metadata is the type source: every result column's
//! type and NOT NULL flag (the server's own analysis, expressions included) and, since 8.0.22, a
//! type per `?` from its context. The engine takes those as they are (see
//! docs/mysql-sources.md). MySQL has no RETURNING and no server surface for the row count, so a
//! data-modifying statement returns no rows, and a SELECT's row count is the unique-key proof
//! over a `sqlparser` tree with the keys read from `information_schema.statistics`.

use crate::api::{Analysis, Column, Diagnostic, MysqlDescribe, Param, Scope};
use crate::jsonshape::Shape;
use crate::dialect::Dialect;
use crate::host::{as_str, Host, Lazy};
use crate::lex::{self, Lexed, Target};
use crate::types::{Field, Type};
use serde_json::json;
use sqlparser::ast::{self, Expr, GroupByExpr, LimitClause, Query, Select, SelectItem, SetExpr, Statement, TableFactor};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use std::collections::{HashMap, HashSet};

/// One row per column of every unique index of the named tables, in key order. `?` is the
/// comma-joined table names, because MySQL binds no arrays.
const UNIQUE_KEYS_QUERY: &str = "select table_name, index_name, column_name from information_schema.statistics where table_schema = database() and non_unique = 0 and find_in_set(table_name, ?) order by table_name, index_name, seq_in_index";

/// The statements the MySQL analysis types. A server prepares DDL too, so a migration would
/// otherwise come back typed as a statement that returns no rows.
const KINDS: &[&str] = &["select", "with", "table", "values", "insert", "replace", "update", "delete"];

pub fn analyze(sql: &str, lexed: &Lexed, host: &Host) -> Lazy<Analysis> {
    if let Err(message) = lex::one_statement(sql, lexed, KINDS) {
        return Some(crate::postgres::bare(sql, lexed, Dialect::Mysql, vec![Diagnostic { message, start: 0, length: sql.trim_end().len().max(1), code: 10 }]));
    }
    let rewritten = lex::rewrite(sql, lexed, Target::Question);
    let raw = host.describe_mysql(&rewritten.sql)?;
    let raw = match raw {
        Ok(r) => r,
        Err(e) => {
            let message = match &e.code {
                Some(code) => format!("{} ({code})", e.message),
                None => e.message.clone(),
            };
            return Some(crate::postgres::bare(sql, lexed, Dialect::Mysql, vec![Diagnostic { message, start: 0, length: sql.trim_end().len().max(1), code: 6 }]));
        }
    };
    let diagnostics = Vec::new();
    let parsed = Parser::parse_sql(&MySqlDialect {}, &rewritten.sql).ok().and_then(|mut v| if v.len() == 1 { v.pop() } else { None });
    // JSON shapes: the constructors' value expressions, prepared as columns of their own statements.
    let mut shaped: Vec<(String, Type, bool)> = Vec::new();
    let mut outer_not_null: Vec<usize> = Vec::new();
    if let Some(Statement::Query(q)) = &parsed {
        let plan = crate::mysql_shape::plan(q);
        let mut outcomes: Vec<Option<MysqlDescribe>> = Vec::with_capacity(plan.probes.len());
        let mut pending = false;
        for probe in &plan.probes {
            let mut outcome = host.describe_mysql(&probe.sql);
            if let (Some(Err(_)), Some(wider)) = (&outcome, &probe.wider) {
                outcome = host.describe_mysql(wider);
            }
            match outcome {
                Some(Ok(d)) => outcomes.push(Some(d)),
                Some(Err(_)) => outcomes.push(None),
                None => {
                    pending = true;
                    outcomes.push(None);
                }
            }
        }
        // The outer columns' flags without the subquery items' pessimism, by projection position.
        let outer = crate::mysql_shape::outer_probe(q);
        if let Some((sql, kept)) = &outer {
            match host.describe_mysql(sql) {
                Some(Ok(d)) => {
                    for (k, &i) in kept.iter().enumerate() {
                        if d.columns.get(k).is_some_and(|c| c.flags & 1 != 0) {
                            outer_not_null.push(i);
                        }
                    }
                }
                Some(Err(_)) => {}
                None => pending = true,
            }
        }
        if pending {
            return None;
        }
        for (name, shape) in &plan.columns {
            let (ty, non_null) = resolve(shape, &outcomes);
            shaped.push((name.clone(), ty, non_null));
        }
    }
    let mut shapes_used: Vec<bool> = vec![false; shaped.len()];
    let columns = raw
        .columns
        .iter()
        .enumerate()
        .map(|(position, c)| {
            let (ty, name) = column_type(c.r#type, c.charset);
            let not_null = c.flags & 1 != 0 || outer_not_null.contains(&position);
            let shape = shaped.iter().enumerate().find(|(i, (n, _, _))| n == &c.name && !shapes_used[*i]);
            match shape {
                // A constructor never returns SQL NULL, whatever the server's flag says; an
                // aggregate over no rows does, unless guarded. A JSON column takes the shape's
                // type; any other keeps the server's and only borrows the flag.
                Some((i, (_, shaped_ty, non_null))) => {
                    shapes_used[i] = true;
                    let ty = if c.r#type == 245 { shaped_ty.clone() } else { ty };
                    Column { name: c.name.clone(), ty, nullable: !(not_null || *non_null), type_name: name.to_string() }
                }
                None => Column { name: c.name.clone(), ty, nullable: !not_null, type_name: name.to_string() },
            }
        })
        .collect();
    // The first `?` a name binds carries the server's type for it.
    let params = lexed
        .params
        .iter()
        .map(|name| {
            let index = rewritten.order.iter().position(|n| n == name);
            let (ty, type_name) = match index.and_then(|i| raw.params.get(i)) {
                Some(p) => param_type(p.r#type, p.charset),
                None => (Type::Unknown, ""),
            };
            // MySQL reports no nullability for a parameter; every one is typed without null.
            Param { name: name.clone(), ty, type_name: type_name.to_string(), nullable: false }
        })
        .collect();
    let kind = match &parsed {
        Some(Statement::Query(_)) => "select",
        Some(Statement::Insert(_)) => "insert",
        Some(Statement::Update(_)) => "update",
        Some(Statement::Delete(_)) => "delete",
        _ => "",
    };
    let (cardinality, reason) = match &parsed {
        Some(Statement::Query(q)) => {
            let tables = tables_of(q);
            let keys = if tables.is_empty() { HashMap::new() } else { unique_keys(host, &tables)? };
            cardinality(q, &keys)
        }
        Some(Statement::Insert(_)) | Some(Statement::Update(_)) | Some(Statement::Delete(_)) => ("none".to_string(), None),
        Some(_) => ("many".to_string(), Some("only a SELECT is proven".into())),
        None => ("many".to_string(), Some("the engine could not parse the statement, so nothing bounds its rows".into())),
    };
    Some(Analysis {
        kind: kind.to_string(),
        params,
        columns,
        cardinality,
        cardinality_reason: reason,
        rewritten,
        style: lexed.style,
        diagnostics,
        unresolved: Vec::new(),
    })
}

/// A result column's protocol type code as the type model, with the MySQL name for messages.
/// The names are the Postgres ones, which the renderer and the runtime conversion already read.
fn column_type(code: u32, charset: u32) -> (Type, &'static str) {
    let scalar = |name: &str| Type::Scalar { name: name.to_string() };
    match code {
        1 => (scalar("int2"), "tinyint"),
        2 => (scalar("int2"), "smallint"),
        3 => (scalar("int4"), "int"),
        9 => (scalar("int4"), "mediumint"),
        13 => (scalar("int2"), "year"),
        8 => (scalar("int8"), "bigint"),
        4 => (scalar("float4"), "float"),
        5 => (scalar("float8"), "double"),
        0 | 246 => (scalar("numeric"), "decimal"),
        7 => (scalar("timestamp"), "timestamp"),
        12 => (scalar("timestamp"), "datetime"),
        10 | 14 => (scalar("date"), "date"),
        11 => (scalar("time"), "time"),
        245 => (Type::Json, "json"),
        16 => (scalar("bytea"), "bit"),
        249..=254 | 15 => {
            if charset == 63 { (scalar("bytea"), "binary") } else { (scalar("text"), "varchar") }
        }
        6 => (Type::Unknown, "null"),
        _ => (Type::Unknown, "unknown"),
    }
}

/// A parameter's type from the server's context guess. Every integer context comes back as
/// LONGLONG, whatever the column's width, so an integer parameter is a plain number.
fn param_type(code: u32, charset: u32) -> (Type, &'static str) {
    match code {
        1 | 2 | 3 | 8 | 9 | 13 => (Type::Scalar { name: "int4".into() }, "int"),
        _ => column_type(code, charset),
    }
}

/// One shape plus its probes' prepares as the column's type and whether it is never NULL.
fn resolve(shape: &Shape, outcomes: &[Option<MysqlDescribe>]) -> (Type, bool) {
    match shape {
        Shape::Object { fields, non_null } => {
            let fields = fields
                .iter()
                .map(|(k, s)| {
                    let (ty, nn) = resolve(s, outcomes);
                    Field { name: k.clone(), ty, nullable: !nn }
                })
                .collect();
            (Type::Object { fields }, *non_null)
        }
        Shape::Array { items, non_null } => {
            let (ty, nn) = resolve(items, outcomes);
            (Type::List { of: Box::new(ty), nullable_items: !nn }, *non_null)
        }
        Shape::Column { probe, index } => match outcomes.get(*probe).and_then(|o| o.as_ref()).and_then(|o| o.columns.get(*index)) {
            Some(c) => (column_type(c.r#type, c.charset).0, c.flags & 1 != 0),
            None => (Type::Unknown, false),
        },
        _ => (Type::Unknown, false),
    }
}

// ---- the row count ------------------------------------------------------------------------------

type Keys = HashMap<String, Vec<Vec<String>>>;

fn unique_keys(host: &Host, tables: &[String]) -> Lazy<Keys> {
    let rows = host.query(UNIQUE_KEYS_QUERY, vec![json!(tables.join(","))], Scope::Statement)?;
    let mut keys: Keys = HashMap::new();
    if let Ok(rows) = rows {
        let mut by_index: Vec<(String, String, Vec<String>)> = Vec::new();
        for row in rows {
            let (Some(table), Some(index), Some(column)) = (row.first().and_then(as_str), row.get(1).and_then(as_str), row.get(2).and_then(as_str)) else { continue };
            match by_index.last_mut() {
                Some((t, i, cols)) if t.eq_ignore_ascii_case(table) && i.eq_ignore_ascii_case(index) => cols.push(column.to_ascii_lowercase()),
                _ => by_index.push((table.to_string(), index.to_string(), vec![column.to_ascii_lowercase()])),
            }
        }
        for (table, _, cols) in by_index {
            keys.entry(table.to_ascii_lowercase()).or_default().push(cols);
        }
    }
    Some(keys)
}

/// A relation the top-level FROM reads: its table name and the name the statement uses for it.
struct Relation {
    table: String,
    alias: String,
}

fn relations(select: &Select) -> Option<Vec<Relation>> {
    let mut out = Vec::new();
    for twj in &select.from {
        for factor in std::iter::once(&twj.relation).chain(twj.joins.iter().map(|j| &j.relation)) {
            match factor {
                TableFactor::Table { name, alias, .. } => {
                    let table = name.0.last().and_then(|p| match p {
                        ast::ObjectNamePart::Identifier(i) => Some(i.value.to_ascii_lowercase()),
                        _ => None,
                    })?;
                    let alias = alias.as_ref().map(|a| a.name.value.to_ascii_lowercase()).unwrap_or_else(|| table.clone());
                    out.push(Relation { table, alias });
                }
                _ => return None,
            }
        }
    }
    Some(out)
}

fn tables_of(q: &Query) -> Vec<String> {
    match q.body.as_ref() {
        SetExpr::Select(s) => relations(s).map(|r| r.into_iter().map(|r| r.table).collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn cardinality(q: &Query, keys: &Keys) -> (String, Option<String>) {
    match exactly_one(q) {
        Ok(()) => ("one".into(), None),
        Err(one_why) => match at_most_one(q, keys) {
            Ok(()) => ("maybeOne".into(), Some(one_why)),
            Err(why) => ("many".into(), Some(why)),
        },
    }
}

fn select_of(q: &Query) -> Result<&Select, String> {
    match q.body.as_ref() {
        SetExpr::Select(s) => Ok(s),
        SetExpr::SetOperation { .. } => Err("a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows".into()),
        SetExpr::Values(_) => Err("a VALUES list returns a row per row it lists".into()),
        _ => Err("only a plain SELECT is proven".into()),
    }
}

fn const_limit(q: &Query) -> Option<Option<i64>> {
    let limit = match &q.limit_clause {
        Some(LimitClause::LimitOffset { limit: Some(l), .. }) => l,
        Some(LimitClause::OffsetCommaLimit { limit, .. }) => limit,
        Some(LimitClause::LimitOffset { limit: None, .. }) | None => return None,
    };
    Some(match limit {
        Expr::Value(v) => match &v.value {
            ast::Value::Number(n, _) => n.parse::<i64>().ok(),
            _ => None,
        },
        _ => None,
    })
}

pub(crate) fn exactly_one(q: &Query) -> Result<(), String> {
    if let SetExpr::Values(v) = q.body.as_ref() {
        return if v.rows.len() == 1 { Ok(()) } else { Err("a VALUES list returns a row per row it lists".into()) };
    }
    let s = select_of(q)?;
    if !matches!(s.group_by, GroupByExpr::Expressions(ref e, _) if e.is_empty()) {
        return Err("GROUP BY returns one row per group, and none when nothing matches".into());
    }
    if s.having.is_some() {
        return Err("HAVING can remove the row".into());
    }
    if q.limit_clause.is_some() {
        return Err("a LIMIT or an OFFSET can remove the row (LIMIT 0, OFFSET 1)".into());
    }
    if s.projection.iter().any(|item| match item {
        SelectItem::UnnamedExpr(e) | SelectItem::ExprWithAlias { expr: e, .. } => has_aggregate(e),
        _ => false,
    }) {
        return Ok(());
    }
    if s.from.is_empty() && s.selection.is_none() {
        return Ok(());
    }
    Err("a FROM returns a row per row it matches, and only an aggregate with no GROUP BY collapses them to one".into())
}

fn at_most_one(q: &Query, keys: &Keys) -> Result<(), String> {
    if let Some(limit) = const_limit(q) {
        return match limit {
            Some(n) if n <= 1 => Ok(()),
            _ => Err("a LIMIT of more than one row, or one the build cannot read, bounds nothing".into()),
        };
    }
    let s = select_of(q)?;
    let Some(relations) = relations(s) else { return Err("only plain tables in the FROM are matched against a unique key".into()) };
    let single = relations.len() == 1;
    let equated = equated_columns(s.selection.as_ref());
    let pinned = |r: &Relation| -> bool {
        let Some(table_keys) = keys.get(&r.table) else { return false };
        table_keys.iter().any(|key| !key.is_empty() && key.iter().all(|c| equated.iter().any(|(alias, col)| owns(alias.as_deref(), col, r, single, table_keys) && col == c)))
    };
    let grouped: Option<&[Expr]> = match &s.group_by {
        GroupByExpr::Expressions(e, _) if !e.is_empty() => Some(e),
        GroupByExpr::All(_) => return Err("GROUP BY ALL returns one row per group".into()),
        _ => None,
    };
    if let Some(group) = grouped {
        for r in &relations {
            if !pinned(r) {
                continue;
            }
            let table_keys = keys.get(&r.table).cloned().unwrap_or_default();
            let own = group.iter().all(|g| match column_ref(g) {
                Some((alias, col)) => owns(alias.as_deref(), &col, r, single, &table_keys) && (alias.is_some() || single || table_keys.iter().flatten().any(|k| *k == col)),
                None => false,
            });
            if own {
                return Ok(());
            }
        }
        return Err("GROUP BY returns one row per group; the engine proves one group only when the WHERE pins a unique key of a relation to values and every GROUP BY column belongs to that relation".into());
    }
    if s.having.is_some() {
        return Err("HAVING leaves a row per group".into());
    }
    let [only] = relations.as_slice() else {
        return Err(match relations.len() {
            0 => "a SELECT with no FROM and a WHERE can return no row or one, and the engine proves that only through a LIMIT".into(),
            _ => "a join returns a row per pair it matches, and the engine proves a single row only for one relation".into(),
        });
    };
    if pinned(only) {
        return Ok(());
    }
    match keys.get(&only.table) {
        None => Err(format!("`{}` has no unique index the engine can read, so nothing bounds the rows a WHERE matches", only.table)),
        Some(table_keys) => {
            let shown: Vec<String> = table_keys.iter().map(|k| format!("({})", k.join(", "))).collect();
            Err(format!("the WHERE does not equate every column of a unique key of `{}` — it has {}", only.table, shown.join(", ")))
        }
    }
}

/// Whether `alias.col` (or a bare `col`) names a column of `r`: a bare name is the relation's
/// when it is the only one, or when it is a column of one of its unique keys.
fn owns(alias: Option<&str>, col: &str, r: &Relation, single: bool, table_keys: &[Vec<String>]) -> bool {
    match alias {
        Some(a) => a == r.alias,
        None => single || table_keys.iter().flatten().any(|k| k == col),
    }
}

/// `(qualifier, column)` of a column reference, lowercased.
fn column_ref(e: &Expr) -> Option<(Option<String>, String)> {
    match e {
        Expr::Identifier(i) => Some((None, i.value.to_ascii_lowercase())),
        Expr::CompoundIdentifier(parts) if parts.len() == 2 => Some((Some(parts[0].value.to_ascii_lowercase()), parts[1].value.to_ascii_lowercase())),
        Expr::Nested(inner) => column_ref(inner),
        _ => None,
    }
}

/// The columns the WHERE equates to a value (no column read on the other side) under ANDs.
fn equated_columns(selection: Option<&Expr>) -> HashSet<(Option<String>, String)> {
    let mut out = HashSet::new();
    let mut stack: Vec<&Expr> = selection.into_iter().collect();
    while let Some(e) = stack.pop() {
        match e {
            Expr::Nested(inner) => stack.push(inner),
            Expr::BinaryOp { left, op: ast::BinaryOperator::And, right } => {
                stack.push(left);
                stack.push(right);
            }
            Expr::BinaryOp { left, op: ast::BinaryOperator::Eq, right } => {
                for (col, other) in [(left, right), (right, left)] {
                    if let Some(c) = column_ref(col) {
                        if !reads_column(other) {
                            out.insert(c);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// Whether an expression reads a column anywhere; anything the walk does not know counts as reading one.
fn reads_column(e: &Expr) -> bool {
    match e {
        Expr::Identifier(_) | Expr::CompoundIdentifier(_) => true,
        Expr::Value(_) | Expr::TypedString { .. } => false,
        Expr::Nested(inner) | Expr::UnaryOp { expr: inner, .. } | Expr::Cast { expr: inner, .. } | Expr::IsNull(inner) | Expr::IsNotNull(inner) => reads_column(inner),
        Expr::BinaryOp { left, right, .. } => reads_column(left) || reads_column(right),
        Expr::Function(f) => match &f.args {
            ast::FunctionArguments::None => false,
            ast::FunctionArguments::List(list) => list.args.iter().any(|a| match a {
                ast::FunctionArg::Unnamed(ast::FunctionArgExpr::Expr(x)) | ast::FunctionArg::Named { arg: ast::FunctionArgExpr::Expr(x), .. } | ast::FunctionArg::ExprNamed { arg: ast::FunctionArgExpr::Expr(x), .. } => reads_column(x),
                _ => true,
            }),
            ast::FunctionArguments::Subquery(_) => true,
        },
        _ => true,
    }
}

const AGGREGATES: &[&str] = &["count", "sum", "avg", "min", "max", "group_concat", "json_arrayagg", "json_objectagg", "bit_and", "bit_or", "bit_xor", "std", "stddev", "stddev_pop", "stddev_samp", "var_pop", "var_samp", "variance"];

/// Whether an expression calls an aggregate at this level: not under OVER, not inside a subquery.
pub(crate) fn has_aggregate(e: &Expr) -> bool {
    match e {
        Expr::Function(f) => {
            let name = f.name.0.last().map(|p| p.to_string().to_ascii_lowercase()).unwrap_or_default();
            if f.over.is_none() && AGGREGATES.contains(&name.as_str()) {
                return true;
            }
            match &f.args {
                ast::FunctionArguments::List(list) => list.args.iter().any(|a| match a {
                    ast::FunctionArg::Unnamed(ast::FunctionArgExpr::Expr(x)) | ast::FunctionArg::Named { arg: ast::FunctionArgExpr::Expr(x), .. } | ast::FunctionArg::ExprNamed { arg: ast::FunctionArgExpr::Expr(x), .. } => has_aggregate(x),
                    _ => false,
                }),
                _ => false,
            }
        }
        Expr::Nested(inner) | Expr::UnaryOp { expr: inner, .. } | Expr::Cast { expr: inner, .. } | Expr::IsNull(inner) | Expr::IsNotNull(inner) => has_aggregate(inner),
        Expr::BinaryOp { left, right, .. } => has_aggregate(left) || has_aggregate(right),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys() -> Keys {
        HashMap::from([("users".to_string(), vec![vec!["id".to_string()], vec!["email".to_string()]]), ("posts".to_string(), vec![vec!["id".to_string()]])])
    }

    fn card(sql: &str) -> String {
        let mut v = Parser::parse_sql(&MySqlDialect {}, sql).unwrap();
        let Statement::Query(q) = v.pop().unwrap() else { panic!("not a query") };
        cardinality(&q, &keys()).0
    }

    #[test]
    fn placeholders_parse_and_keys_prove() {
        assert_eq!(card("select id, name from users where id = ? and name <> ?"), "maybeOne");
        assert_eq!(card("select id from users where email = ?"), "maybeOne");
        assert_eq!(card("select id from users where name = ?"), "many");
        assert_eq!(card("select count(*) as n, max(at) as latest from users where name like ?"), "one");
        assert_eq!(card("select u.id, count(p.id) as n from users u left join posts p on p.user_id = u.id where u.id = ? group by u.id"), "maybeOne");
        assert_eq!(card("select id from users limit 1"), "maybeOne");
        assert_eq!(card("select id from users limit 5, 1"), "maybeOne");
        assert_eq!(card("select id from users union all select user_id from posts"), "many");
        assert_eq!(card("select id from users where id = ? + 1"), "maybeOne");
        assert_eq!(card("select id from users u join posts p on p.user_id = u.id where u.id = p.user_id"), "many");
    }
}
