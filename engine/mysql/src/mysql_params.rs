//! Which MySQL parameters the caller may pass null for, by the rules of Postgres and SQLite
//! (`postgres::nullable_params`, `sqlite::guarded_params`), read off the `sqlparser` tree, since
//! MySQL's prepare says nothing of a parameter's nullability: one the statement tests for null
//! (`:p is null`, `:p is not null`, `isnull(:p)`, `:p <=> x`) or passes first to `coalesce` or
//! `ifnull`, directly or through the column of a WITH query or a derived table that selects it
//! (`with input as (select :p as p) … where input.p is null`), and one that is the whole value
//! written to a column the table lets hold null (a row of an INSERT's or a REPLACE's VALUES, its
//! SET list, an UPDATE's SET, ON DUPLICATE KEY UPDATE), unless it also fills a NOT NULL column.
//! Every other parameter is a non-null value.

use crate::api::Scope;
use crate::fresh::Tables;
use crate::host::{as_str, Host, Lazy};
use crate::lex;
use serde_json::json;
use sqlparser::ast::{self, Expr, FunctionArg, FunctionArgExpr, Query, Select, SelectItem, SetExpr, Statement, TableFactor, TableWithJoins};
use std::collections::{HashMap, HashSet};

/// Each column of the named tables of the current schema, whether it may hold NULL, and its
/// `extra`, in the order an INSERT without a column list fills them, which passes over an
/// INVISIBLE column. `?` is the comma-joined table names, because MySQL binds no arrays.
const COLUMNS_QUERY: &str = "select c.table_name, c.column_name, c.is_nullable, c.extra from information_schema.columns c \
    where c.table_schema = database() and find_in_set(c.table_name, ?) order by c.table_name, c.ordinal_position";

/// Each column's type of the named tables of the current schema, for [`column_types`]: its
/// `data_type`, its `column_type` (`tinyint(1)`, `bit(1)`, `bigint unsigned`) and its `extra`.
const TYPES_QUERY: &str = "select c.table_name, c.column_name, c.data_type, c.column_type, c.extra from information_schema.columns c \
    where c.table_schema = database() and find_in_set(c.table_name, ?) order by c.table_name, c.ordinal_position";

/// The parameter each `?` of the rewritten text is, by where it stands as the parser counts:
/// lines and characters from 1.
pub(crate) type Names = HashMap<(u64, u64), String>;

pub(crate) fn names(rewritten: &lex::Rewritten) -> Names {
    let mut out = Names::new();
    for (i, &(_, _, start, _)) in rewritten.spans.iter().enumerate() {
        let before = &rewritten.sql[..start.min(rewritten.sql.len())];
        let line = before.matches('\n').count() as u64 + 1;
        let column = before.rsplit('\n').next().map_or(0, |l| l.chars().count()) as u64 + 1;
        if let Some(name) = rewritten.order.get(i) {
            out.insert((line, column), name.clone());
        }
    }
    out
}

/// Asks for the columns of the tables the statement writes a parameter to, so that the answer
/// comes in the round of the statement's own prepare.
pub(crate) fn prefetch(statement: &Statement, names: &Names, host: &Host) {
    let tables = written_tables(&filled(statement, names));
    if !tables.is_empty() {
        let _ = host.query(COLUMNS_QUERY, vec![json!(tables.join(","))], Scope::Statement);
    }
    let typed = typed_targets(statement, names);
    if !typed.is_empty() {
        let _ = host.query(TYPES_QUERY, vec![json!(written_tables(&typed).join(","))], Scope::Statement);
    }
}

/// The parameters written whole to a column or compared whole with one, each with that column.
fn typed_targets(statement: &Statement, names: &Names) -> Vec<(String, String, Target)> {
    let mut targets = filled(statement, names);
    targets.extend(compared(statement, names));
    targets
}

/// The type of each parameter whose column the statement names directly, where it differs from
/// what MySQL's prepare says: the server describes every integer context as a BIGINT and a BIT
/// context as BIT(64), whatever the column, so a parameter written to or compared with a BIGINT
/// column is `int8`, one with a TINYINT(1) or BIT(1) column, which a row returns as a truth
/// value, `bool|int4`, and one with a wider BIT column `bytea`. A row's value then goes back in
/// as a parameter of its type (the round-trip rule, the maintainer, 2026-10-20). A parameter whose
/// columns disagree keeps the prepare's type.
pub(crate) fn column_types(statement: &Statement, names: &Names, host: &Host) -> Lazy<HashMap<String, &'static str>> {
    let targets = typed_targets(statement, names);
    if targets.is_empty() {
        return Some(HashMap::new());
    }
    let rows = host.query(TYPES_QUERY, vec![json!(written_tables(&targets).join(","))], Scope::Statement)?;
    // Per table as the server names it: each column in lower case, its type, and whether it is INVISIBLE.
    let mut columns: HashMap<String, Vec<(String, Option<&'static str>, bool)>> = HashMap::new();
    for row in rows.unwrap_or_default() {
        let (Some(table), Some(column), Some(data_type), Some(column_type)) = (row.first().and_then(as_str), row.get(1).and_then(as_str), row.get(2).and_then(as_str), row.get(3).and_then(as_str)) else { continue };
        let invisible = row.get(4).and_then(as_str).is_some_and(|extra| extra.to_ascii_uppercase().contains("INVISIBLE"));
        let column_type = column_type.to_ascii_lowercase();
        let ty = match data_type.to_ascii_lowercase().as_str() {
            "bigint" => Some("int8"),
            "tinyint" if column_type.starts_with("tinyint(1)") => Some("bool|int4"),
            "bit" if column_type.starts_with("bit(1)") => Some("bool|int4"),
            "bit" => Some("bytea"),
            _ => None,
        };
        columns.entry(table.to_string()).or_default().push((column.to_ascii_lowercase(), ty, invisible));
    }
    let mut found: HashMap<String, Option<&'static str>> = HashMap::new();
    for (param, table, target) in targets {
        let known = match columns.get(&table) {
            Some(known) => Some(known),
            None => {
                let mut same = columns.iter().filter(|(name, _)| name.eq_ignore_ascii_case(&table));
                match (same.next(), same.next()) {
                    (Some((_, known)), None) => Some(known),
                    _ => None,
                }
            }
        };
        let ty = known.and_then(|known| match &target {
            Target::Name(name) => known.iter().find(|(c, _, _)| c == name).map(|(_, ty, _)| *ty),
            Target::Position(i) => known.iter().filter(|(_, _, invisible)| !invisible).nth(*i).map(|(_, ty, _)| *ty),
        });
        // A column nothing narrows, or one the statement names that the table lacks, keeps the prepare's type.
        let ty = ty.flatten();
        found.entry(param).and_modify(|seen| if *seen != ty { *seen = None }).or_insert(ty);
    }
    Some(found.into_iter().filter_map(|(param, ty)| ty.map(|ty| (param, ty))).collect())
}

/// Every parameter compared whole with a column of the table the statement reads, in its WHERE
/// or its HAVING (`col = :p`, `<>`, `<`, `<=`, `>`, `>=`, `<=>`, `col in (:a, :b)`, `col between
/// :a and :b`): (parameter, table as the statement names it, column). A bare column is the one
/// table's where the statement reads one; a qualified one is the table its name or alias names.
fn compared(statement: &Statement, names: &Names) -> Vec<(String, String, Target)> {
    let mut out = Vec::new();
    let (from, filters): (Vec<&TableWithJoins>, Vec<&Expr>) = match statement {
        Statement::Query(q) => match q.body.as_ref() {
            SetExpr::Select(s) => (s.from.iter().collect(), s.selection.iter().chain(s.having.iter()).collect()),
            _ => return out,
        },
        Statement::Update(update) => (std::iter::once(&update.table).collect(), update.selection.iter().collect()),
        Statement::Delete(delete) => {
            let (ast::FromTable::WithFromKeyword(from) | ast::FromTable::WithoutKeyword(from)) = &delete.from;
            (from.iter().collect(), delete.selection.iter().collect())
        }
        _ => return out,
    };
    // The tables by the name the statement reads each by; `plain` where every item is a table.
    let mut tables: Vec<(String, String)> = Vec::new();
    let mut plain = true;
    for twj in &from {
        for factor in std::iter::once(&twj.relation).chain(twj.joins.iter().map(|j| &j.relation)) {
            match factor {
                TableFactor::Table { name, alias, .. } => match table_name(name) {
                    Some(table) => {
                        let by = alias.as_ref().map(|a| a.name.value.clone()).unwrap_or_else(|| table.clone());
                        tables.push((by.to_ascii_lowercase(), table));
                    }
                    None => plain = false,
                },
                _ => plain = false,
            }
        }
    }
    let column = |e: &Expr| -> Option<(String, String)> {
        match e {
            Expr::Identifier(i) if plain && tables.len() == 1 => Some((tables[0].1.clone(), i.value.to_ascii_lowercase())),
            Expr::CompoundIdentifier(parts) if parts.len() == 2 => {
                let by = parts[0].value.to_ascii_lowercase();
                tables.iter().find(|(b, _)| *b == by).map(|(_, t)| (t.clone(), parts[1].value.to_ascii_lowercase()))
            }
            _ => None,
        }
    };
    fn walk(e: &Expr, names: &Names, column: &dyn Fn(&Expr) -> Option<(String, String)>, out: &mut Vec<(String, String, Target)>) {
        use ast::BinaryOperator::*;
        let pair = |col: &Expr, p: &Expr, out: &mut Vec<(String, String, Target)>| {
            if let (Some((table, name)), Some(param)) = (column(col), direct(p, names)) {
                out.push((param, table, Target::Name(name)));
            }
        };
        match e {
            Expr::BinaryOp { left, op: Eq | NotEq | Lt | LtEq | Gt | GtEq | Spaceship, right } => {
                pair(left, right, out);
                pair(right, left, out);
            }
            Expr::BinaryOp { left, op: And | Or | Xor, right } => {
                walk(left, names, column, out);
                walk(right, names, column, out);
            }
            Expr::InList { expr, list, .. } => {
                for item in list {
                    pair(expr, item, out);
                }
            }
            Expr::Between { expr, low, high, .. } => {
                pair(expr, low, out);
                pair(expr, high, out);
            }
            Expr::Nested(inner) | Expr::UnaryOp { expr: inner, .. } => walk(inner, names, column, out),
            _ => {}
        }
    }
    for filter in filters {
        walk(filter, names, &column, &mut out);
    }
    out
}

/// The parameters of `statement` the caller may pass null for, by name.
pub(crate) fn nullable(statement: &Statement, names: &Names, host: &Host) -> Lazy<HashSet<String>> {
    let mut walk = Walk { names, out: HashSet::new(), tables: Vec::new() };
    walk.statement(statement, &Env::default());
    let mut out = walk.out;
    let filled = filled(statement, names);
    if filled.is_empty() {
        return Some(out);
    }
    let rows = host.query(COLUMNS_QUERY, vec![json!(written_tables(&filled).join(","))], Scope::Statement)?;
    // Per table as the server names it: each column in lower case, whether it may hold NULL, and
    // whether it is INVISIBLE.
    let mut columns: HashMap<String, Vec<(String, bool, bool)>> = HashMap::new();
    for row in rows.unwrap_or_default() {
        let (Some(table), Some(column), Some(nullable)) = (row.first().and_then(as_str), row.get(1).and_then(as_str), row.get(2).and_then(as_str)) else { continue };
        let invisible = row.get(3).and_then(as_str).is_some_and(|extra| extra.to_ascii_uppercase().contains("INVISIBLE"));
        columns.entry(table.to_string()).or_default().push((column.to_ascii_lowercase(), nullable.eq_ignore_ascii_case("YES"), invisible));
    }
    let mut fills: HashMap<String, bool> = HashMap::new();
    for (param, table, target) in filled {
        // The server's name for the table: the query compares names as the server does, so a
        // row under another case came back because the server holds the two for one table.
        let known = match columns.get(&table) {
            Some(known) => Some(known),
            None => {
                let mut same = columns.iter().filter(|(name, _)| name.eq_ignore_ascii_case(&table));
                match (same.next(), same.next()) {
                    (Some((_, known)), None) => Some(known),
                    _ => None,
                }
            }
        };
        let Some(known) = known else { continue };
        let nullable = match &target {
            Target::Name(name) => known.iter().find(|(c, _, _)| c == name).map(|(_, nullable, _)| *nullable),
            Target::Position(i) => known.iter().filter(|(_, _, invisible)| !invisible).nth(*i).map(|(_, nullable, _)| *nullable),
        };
        if let Some(nullable) = nullable {
            *fills.entry(param).or_insert(true) &= nullable;
        }
    }
    out.extend(fills.into_iter().filter(|(_, nullable)| *nullable).map(|(param, _)| param));
    Some(out)
}

/// The tables of the current schema the statement reads and writes, as it names them, for the
/// freshness facts ([`crate::fresh`]): every table a FROM item reads, in subqueries and WITH
/// queries too, and the table an INSERT, an UPDATE or a DELETE writes. A WITH query is no table, and
/// a table of another schema is left out, since the facts are read from the current one.
pub(crate) fn tables(statement: &Statement) -> Tables {
    let names = Names::new();
    let mut walk = Walk { names: &names, out: HashSet::new(), tables: Vec::new() };
    walk.statement(statement, &Env::default());
    let target = |relation: &TableFactor| match relation {
        TableFactor::Table { name, .. } => table_name(name),
        _ => None,
    };
    let write = match statement {
        Statement::Insert(insert) => match &insert.table {
            ast::TableObject::TableName(name) => table_name(name).into_iter().collect(),
            _ => Vec::new(),
        },
        Statement::Update(update) => target(&update.table.relation).into_iter().collect(),
        Statement::Delete(delete) if delete.tables.is_empty() => {
            let (ast::FromTable::WithFromKeyword(from) | ast::FromTable::WithoutKeyword(from)) = &delete.from;
            from.first().and_then(|t| target(&t.relation)).into_iter().collect()
        }
        _ => Vec::new(),
    };
    Tables { read: walk.tables, write }
}

/// The column a parameter is written to whole, by name or by its place in the table.
#[derive(Debug, PartialEq)]
enum Target {
    Name(String),
    Position(usize),
}

fn written_tables(filled: &[(String, String, Target)]) -> Vec<String> {
    let mut tables: Vec<String> = filled.iter().map(|(_, table, _)| table.clone()).collect();
    tables.sort();
    tables.dedup();
    tables
}

/// The table name a statement writes to, when it is one of the current schema.
fn table_name(name: &ast::ObjectName) -> Option<String> {
    match name.0.as_slice() {
        [ast::ObjectNamePart::Identifier(i)] => Some(i.value.clone()),
        _ => None,
    }
}

/// The parameter an expression is, whole: under casts and parentheses, which leave a NULL as it is.
fn direct(e: &Expr, names: &Names) -> Option<String> {
    match e {
        Expr::Value(v) => match &v.value {
            ast::Value::Placeholder(p) if p == "?" => names.get(&(v.span.start.line, v.span.start.column)).cloned(),
            _ => None,
        },
        Expr::Nested(inner) | Expr::Cast { expr: inner, .. } | Expr::Convert { expr: inner, .. } => direct(inner, names),
        _ => None,
    }
}

/// Every parameter written whole to a column: (parameter, table as the statement names it, column).
fn filled(statement: &Statement, names: &Names) -> Vec<(String, String, Target)> {
    let mut out = Vec::new();
    let column = |target: &ast::AssignmentTarget| match target {
        ast::AssignmentTarget::ColumnName(name) => name.0.last().and_then(|p| p.as_ident()).map(|i| (name.0.len(), i.value.to_ascii_lowercase())),
        ast::AssignmentTarget::Tuple(_) => None,
    };
    match statement {
        Statement::Insert(insert) => {
            let ast::TableObject::TableName(name) = &insert.table else { return out };
            let Some(table) = table_name(name) else { return out };
            let columns: Vec<String> = insert.columns.iter().filter_map(|c| c.0.last().and_then(|p| p.as_ident()).map(|i| i.value.to_ascii_lowercase())).collect();
            if columns.len() != insert.columns.len() {
                return out;
            }
            if let Some(SetExpr::Values(values)) = insert.source.as_ref().filter(|q| q.with.is_none()).map(|q| q.body.as_ref()) {
                for row in &values.rows {
                    for (i, value) in row.content.iter().enumerate() {
                        let Some(param) = direct(value, names) else { continue };
                        let target = if columns.is_empty() { Some(Target::Position(i)) } else { columns.get(i).map(|c| Target::Name(c.clone())) };
                        if let Some(target) = target {
                            out.push((param, table.clone(), target));
                        }
                    }
                }
            }
            let update = match &insert.on {
                Some(ast::OnInsert::DuplicateKeyUpdate(set)) => set.as_slice(),
                _ => &[],
            };
            for a in insert.assignments.iter().chain(update) {
                if let (Some(param), Some((_, name))) = (direct(&a.value, names), column(&a.target)) {
                    out.push((param, table.clone(), Target::Name(name)));
                }
            }
        }
        Statement::Update(update) => {
            // The tables the statement names, by the name it uses for each.
            let mut tables: Vec<(String, String)> = Vec::new();
            let mut plain = update.from.is_none() && update.table.joins.is_empty();
            for factor in std::iter::once(&update.table.relation).chain(update.table.joins.iter().map(|j| &j.relation)) {
                match factor {
                    TableFactor::Table { name, alias, .. } => match table_name(name) {
                        Some(table) => {
                            let by = alias.as_ref().map(|a| a.name.value.clone()).unwrap_or_else(|| table.clone());
                            tables.push((by.to_ascii_lowercase(), table));
                        }
                        None => plain = false,
                    },
                    _ => plain = false,
                }
            }
            for a in &update.assignments {
                let (Some(param), Some((parts, name))) = (direct(&a.value, names), column(&a.target)) else { continue };
                let table = match (&a.target, parts) {
                    // A bare column is the one table's, where there is one.
                    (_, 1) if plain => tables.first().map(|(_, t)| t.clone()),
                    (ast::AssignmentTarget::ColumnName(n), 2) => {
                        let by = n.0.first().and_then(|p| p.as_ident()).map(|i| i.value.to_ascii_lowercase());
                        tables.iter().find(|(b, _)| Some(b) == by.as_ref()).map(|(_, t)| t.clone())
                    }
                    _ => None,
                };
                if let Some(table) = table {
                    out.push((param, table, Target::Name(name)));
                }
            }
        }
        _ => {}
    }
    out
}

/// A query's columns in order, each with the name a statement reads it by, if it has one, and
/// the parameter it selects whole, if it does; `whole` where no wildcard stands for some of them.
#[derive(Clone, Default)]
struct Columns {
    list: Vec<(Option<String>, Option<String>)>,
    whole: bool,
}

impl Columns {
    /// The columns under the names a column list gives them (`with input (q) as (select :q)`).
    fn renamed(self, names: &[ast::TableAliasColumnDef]) -> Columns {
        if names.is_empty() {
            return self;
        }
        let keep = self.whole && self.list.len() == names.len();
        let list = names.iter().enumerate().map(|(i, n)| (Some(n.name.value.to_ascii_lowercase()), self.list.get(i).and_then(|(_, p)| p.clone()).filter(|_| keep))).collect();
        Columns { list, whole: true }
    }

    fn get(&self, column: &str) -> Option<&Option<String>> {
        self.list.iter().find(|(c, _)| c.as_deref() == Some(column)).map(|(_, p)| p)
    }

    /// Whether every column has its name here, so that a bare name absent from it is not the query's.
    fn complete(&self) -> bool {
        self.whole && self.list.iter().all(|(c, _)| c.is_some())
    }
}

/// A FROM item: the name the statement qualifies its columns with, in lower case, and, for a WITH
/// query or a derived table, its columns; a table's are not known here.
#[derive(Clone)]
struct Item {
    name: String,
    columns: Option<Columns>,
}

/// What a place in the statement sees: the WITH queries by name, the latest last, and the FROM
/// items of each enclosing SELECT, the innermost last.
#[derive(Clone, Default)]
struct Env {
    ctes: Vec<(String, Option<Columns>)>,
    levels: Vec<Vec<Item>>,
}

struct Walk<'n> {
    names: &'n Names,
    out: HashSet<String>,
    /// Each table of the current schema a FROM item reads, as the statement names it.
    tables: Vec<String>,
}

impl Walk<'_> {
    /// The parameter an operand is, under casts, or that the column it reads selects.
    fn param_of(&self, e: &Expr, env: &Env) -> Option<String> {
        match e {
            Expr::Nested(inner) | Expr::Cast { expr: inner, .. } | Expr::Convert { expr: inner, .. } => self.param_of(inner, env),
            Expr::CompoundIdentifier(parts) if parts.len() == 2 => {
                let (by, column) = (parts[0].value.to_ascii_lowercase(), parts[1].value.to_ascii_lowercase());
                let item = env.levels.iter().rev().find_map(|level| level.iter().find(|item| item.name == by))?;
                item.columns.as_ref()?.get(&column)?.clone()
            }
            // A bare name is the column of the innermost SELECT whose FROM has it: of the one
            // WITH query or derived table there that has it (a table that had it too would make
            // the name ambiguous, which the prepare refuses), and of no level past a table,
            // whose columns are not known here.
            Expr::Identifier(ident) => {
                let column = ident.value.to_ascii_lowercase();
                for level in env.levels.iter().rev() {
                    let mut having = level.iter().filter_map(|item| item.columns.as_ref().filter(|c| c.complete())?.get(&column));
                    match (having.next(), having.next()) {
                        (Some(param), None) => return param.clone(),
                        (Some(_), Some(_)) => return None,
                        _ => {}
                    }
                    if level.iter().any(|item| item.columns.as_ref().is_none_or(|c| !c.complete())) {
                        return None;
                    }
                }
                None
            }
            _ => direct(e, self.names),
        }
    }

    fn mark(&mut self, e: &Expr, env: &Env) {
        if let Some(param) = self.param_of(e, env) {
            self.out.insert(param);
        }
    }

    fn statement(&mut self, statement: &Statement, env: &Env) {
        match statement {
            Statement::Query(q) => {
                self.query(q, env);
            }
            Statement::Insert(insert) => {
                if let Some(source) = &insert.source {
                    self.query(source, env);
                }
                let update = match &insert.on {
                    Some(ast::OnInsert::DuplicateKeyUpdate(set)) => set.as_slice(),
                    _ => &[],
                };
                for a in insert.assignments.iter().chain(update) {
                    self.expr(&a.value, env);
                }
            }
            Statement::Update(update) => {
                let mut from = vec![update.table.clone()];
                if let Some(ast::UpdateTableFromKind::BeforeSet(more) | ast::UpdateTableFromKind::AfterSet(more)) = &update.from {
                    from.extend(more.iter().cloned());
                }
                let env = self.level(&from, env);
                for a in &update.assignments {
                    self.expr(&a.value, &env);
                }
                for e in update.selection.iter().chain(update.order_by.iter().map(|o| &o.expr)).chain(update.limit.iter()) {
                    self.expr(e, &env);
                }
            }
            Statement::Delete(delete) => {
                let (ast::FromTable::WithFromKeyword(from) | ast::FromTable::WithoutKeyword(from)) = &delete.from;
                let mut from = from.clone();
                from.extend(delete.using.iter().flatten().cloned());
                let env = self.level(&from, env);
                for e in delete.selection.iter().chain(delete.order_by.iter().map(|o| &o.expr)).chain(delete.limit.iter()) {
                    self.expr(e, &env);
                }
            }
            _ => {}
        }
    }

    /// Walks a query and returns its columns.
    fn query(&mut self, q: &Query, env: &Env) -> Option<Columns> {
        let mut env = env.clone();
        for cte in q.with.iter().flat_map(|w| &w.cte_tables) {
            let columns = self.query(&cte.query, &env).map(|c| c.renamed(&cte.alias.columns));
            env.ctes.push((cte.alias.name.value.to_ascii_lowercase(), columns));
        }
        let (inner, columns) = self.body(&q.body, &env);
        let inner = inner.unwrap_or(env);
        for o in q.order_by.iter().flat_map(|o| match &o.kind {
            ast::OrderByKind::Expressions(list) => list.as_slice(),
            _ => &[],
        }) {
            self.expr(&o.expr, &inner);
        }
        columns
    }

    /// Walks a query's body; a SELECT gives what its ORDER BY sees, and its columns.
    fn body(&mut self, body: &SetExpr, env: &Env) -> (Option<Env>, Option<Columns>) {
        match body {
            SetExpr::Select(s) => {
                let (inner, columns) = self.select(s, env);
                (Some(inner), Some(columns))
            }
            SetExpr::Query(q) => (None, self.query(q, env)),
            SetExpr::SetOperation { left, right, .. } => {
                self.body(left, env);
                self.body(right, env);
                (None, None)
            }
            SetExpr::Values(values) => {
                for e in values.rows.iter().flat_map(|row| &row.content) {
                    self.expr(e, env);
                }
                (None, None)
            }
            SetExpr::Insert(s) | SetExpr::Update(s) | SetExpr::Delete(s) | SetExpr::Merge(s) => {
                self.statement(s, env);
                (None, None)
            }
            _ => (None, None),
        }
    }

    fn select(&mut self, s: &Select, env: &Env) -> (Env, Columns) {
        let inner = self.level(&s.from, env);
        let mut columns = Columns { list: Vec::new(), whole: true };
        for item in &s.projection {
            match item {
                SelectItem::ExprWithAlias { expr, alias } => {
                    self.expr(expr, &inner);
                    columns.list.push((Some(alias.value.to_ascii_lowercase()), self.param_of(expr, &inner)));
                }
                SelectItem::UnnamedExpr(expr) => {
                    self.expr(expr, &inner);
                    // Any other expression MySQL names by its text, which no statement reads it by.
                    let name = match expr {
                        Expr::Identifier(i) => Some(i.value.to_ascii_lowercase()),
                        Expr::CompoundIdentifier(parts) => parts.last().map(|i| i.value.to_ascii_lowercase()),
                        _ => None,
                    };
                    columns.list.push((name, self.param_of(expr, &inner)));
                }
                _ => columns.whole = false,
            }
        }
        let group_by = match &s.group_by {
            ast::GroupByExpr::Expressions(list, _) => list.as_slice(),
            _ => &[],
        };
        for e in s.selection.iter().chain(group_by).chain(s.having.iter()).chain(s.qualify.iter()) {
            self.expr(e, &inner);
        }
        (inner, columns)
    }

    /// The FROM items of a SELECT, as the level its expressions see; the subqueries and the ON
    /// conditions in them walked.
    fn level(&mut self, from: &[TableWithJoins], env: &Env) -> Env {
        let mut items: Vec<Item> = Vec::new();
        let mut on: Vec<&Expr> = Vec::new();
        for twj in from {
            self.items(twj, env, &mut items, &mut on);
        }
        let mut inner = env.clone();
        inner.levels.push(items);
        for e in on {
            self.expr(e, &inner);
        }
        inner
    }

    fn items<'t>(&mut self, twj: &'t TableWithJoins, env: &Env, items: &mut Vec<Item>, on: &mut Vec<&'t Expr>) {
        self.factor(&twj.relation, env, items, on);
        for join in &twj.joins {
            self.factor(&join.relation, env, items, on);
            use ast::JoinOperator as J;
            let constraint = match &join.join_operator {
                J::Join(c) | J::Inner(c) | J::Left(c) | J::LeftOuter(c) | J::Right(c) | J::RightOuter(c) | J::FullOuter(c) | J::CrossJoin(c) | J::Semi(c) | J::LeftSemi(c) | J::RightSemi(c) | J::Anti(c) | J::LeftAnti(c) | J::RightAnti(c) | J::StraightJoin(c) => Some(c),
                J::AsOf { constraint, .. } => Some(constraint),
                _ => None,
            };
            if let Some(ast::JoinConstraint::On(e)) = constraint {
                on.push(e);
            }
        }
    }

    fn factor<'t>(&mut self, factor: &'t TableFactor, env: &Env, items: &mut Vec<Item>, on: &mut Vec<&'t Expr>) {
        let alias_of = |alias: &Option<ast::TableAlias>| alias.as_ref().map(|a| a.name.value.to_ascii_lowercase());
        match factor {
            TableFactor::Table { name, alias, .. } => {
                let table = table_name(name).map(|t| t.to_ascii_lowercase());
                // A WITH query in scope is what a bare name in FROM reads, whatever table shares it.
                let cte = table.as_ref().and_then(|t| env.ctes.iter().rev().find(|(n, _)| n == t));
                if cte.is_none() {
                    self.tables.extend(table_name(name));
                }
                let columns = cte.map(|(_, c)| c.clone().map(|c| c.renamed(alias.as_ref().map_or(&[], |a| a.columns.as_slice()))));
                let by = alias_of(alias).or(table).or_else(|| name.0.last().and_then(|p| p.as_ident()).map(|i| i.value.to_ascii_lowercase())).unwrap_or_default();
                items.push(Item { name: by, columns: columns.flatten() });
            }
            TableFactor::Derived { subquery, alias, lateral, .. } => {
                // A LATERAL one sees the items before it.
                let seen = match lateral {
                    true => {
                        let mut seen = env.clone();
                        seen.levels.push(items.clone());
                        seen
                    }
                    false => env.clone(),
                };
                let columns = self.query(subquery, &seen).map(|c| c.renamed(alias.as_ref().map_or(&[], |a| a.columns.as_slice())));
                items.push(Item { name: alias_of(alias).unwrap_or_default(), columns });
            }
            TableFactor::NestedJoin { table_with_joins, .. } => self.items(table_with_joins, env, items, on),
            TableFactor::JsonTable { json_expr, alias, .. } => {
                self.expr(json_expr, env);
                items.push(Item { name: alias_of(alias).unwrap_or_default(), columns: None });
            }
            TableFactor::TableFunction { expr, alias } => {
                self.expr(expr, env);
                items.push(Item { name: alias_of(alias).unwrap_or_default(), columns: None });
            }
            _ => items.push(Item { name: String::new(), columns: None }),
        }
    }

    /// Walks an expression: a null test or a coalesce of a parameter marks it.
    fn expr(&mut self, e: &Expr, env: &Env) {
        match e {
            Expr::IsNull(x) | Expr::IsNotNull(x) => self.mark(x, env),
            Expr::IsDistinctFrom(a, b) | Expr::IsNotDistinctFrom(a, b) | Expr::BinaryOp { left: a, op: ast::BinaryOperator::Spaceship, right: b } => {
                self.mark(a, env);
                self.mark(b, env);
            }
            Expr::Function(f) if matches!(crate::mysql::builtin(f).as_str(), "coalesce" | "ifnull" | "isnull") => {
                if let ast::FunctionArguments::List(list) = &f.args {
                    if let Some(first) = list.args.first().and_then(arg_expr) {
                        self.mark(first, env);
                    }
                }
            }
            _ => {}
        }
        let mut children: Vec<&Expr> = Vec::new();
        let mut queries: Vec<&Query> = Vec::new();
        match e {
            Expr::IsFalse(x) | Expr::IsNotFalse(x) | Expr::IsTrue(x) | Expr::IsNotTrue(x) | Expr::IsNull(x) | Expr::IsNotNull(x) | Expr::IsUnknown(x) | Expr::IsNotUnknown(x) | Expr::Nested(x) => children.push(x),
            Expr::IsDistinctFrom(a, b) | Expr::IsNotDistinctFrom(a, b) => children.extend([a.as_ref(), b.as_ref()]),
            Expr::BinaryOp { left, right, .. } | Expr::AnyOp { left, right, .. } | Expr::AllOp { left, right, .. } => children.extend([left.as_ref(), right.as_ref()]),
            Expr::UnaryOp { expr, .. } | Expr::Cast { expr, .. } | Expr::Collate { expr, .. } | Expr::Extract { expr, .. } | Expr::Ceil { expr, .. } | Expr::Floor { expr, .. } => children.push(expr),
            Expr::Convert { expr, styles, .. } => {
                children.push(expr);
                children.extend(styles);
            }
            Expr::InList { expr, list, .. } => {
                children.push(expr);
                children.extend(list);
            }
            Expr::InSubquery { expr, subquery, .. } => {
                children.push(expr);
                queries.push(subquery);
            }
            Expr::Between { expr, low, high, .. } => children.extend([expr.as_ref(), low.as_ref(), high.as_ref()]),
            Expr::Like { expr, pattern, escape_char, .. } | Expr::ILike { expr, pattern, escape_char, .. } | Expr::SimilarTo { expr, pattern, escape_char, .. } => {
                children.extend([expr.as_ref(), pattern.as_ref()]);
                children.extend(escape_char.as_deref());
            }
            Expr::RLike { expr, pattern, .. } => children.extend([expr.as_ref(), pattern.as_ref()]),
            Expr::AtTimeZone { timestamp, time_zone } => children.extend([timestamp.as_ref(), time_zone.as_ref()]),
            Expr::Position { expr, r#in } => children.extend([expr.as_ref(), r#in.as_ref()]),
            Expr::Substring { expr, substring_from, substring_for, .. } => {
                children.push(expr);
                children.extend(substring_from.as_deref());
                children.extend(substring_for.as_deref());
            }
            Expr::Trim { expr, trim_what, trim_characters, .. } => {
                children.push(expr);
                children.extend(trim_what.as_deref());
                children.extend(trim_characters.iter().flatten());
            }
            Expr::Overlay { expr, overlay_what, overlay_from, overlay_for } => {
                children.extend([expr.as_ref(), overlay_what.as_ref(), overlay_from.as_ref()]);
                children.extend(overlay_for.as_deref());
            }
            Expr::Function(f) => {
                for args in [&f.parameters, &f.args] {
                    match args {
                        ast::FunctionArguments::List(list) => children.extend(list.args.iter().filter_map(arg_expr)),
                        ast::FunctionArguments::Subquery(q) => queries.push(q),
                        ast::FunctionArguments::None => {}
                    }
                }
                children.extend(f.filter.as_deref());
                children.extend(f.within_group.iter().map(|o| &o.expr));
            }
            Expr::Case { operand, conditions, else_result, .. } => {
                children.extend(operand.as_deref());
                for when in conditions {
                    children.extend([&when.condition, &when.result]);
                }
                children.extend(else_result.as_deref());
            }
            Expr::Exists { subquery, .. } => queries.push(subquery),
            Expr::Subquery(q) => queries.push(q),
            Expr::Tuple(list) => children.extend(list),
            Expr::Interval(i) => children.push(&i.value),
            Expr::JsonAccess { value, .. } => children.push(value),
            Expr::CompoundFieldAccess { root, .. } => children.push(root),
            Expr::MemberOf(m) => children.extend([m.value.as_ref(), m.array.as_ref()]),
            Expr::Prefixed { value, .. } => children.push(value),
            _ => {}
        }
        for child in children {
            self.expr(child, env);
        }
        for q in queries {
            self.query(q, env);
        }
    }
}

fn arg_expr(a: &FunctionArg) -> Option<&Expr> {
    match a {
        FunctionArg::Unnamed(FunctionArgExpr::Expr(x)) | FunctionArg::Named { arg: FunctionArgExpr::Expr(x), .. } | FunctionArg::ExprNamed { arg: FunctionArgExpr::Expr(x), .. } => Some(x),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{Answer, Need};
    use crate::host::Host;
    use serde_json::json;
    use std::collections::HashMap;

    /// The analysis of `sql` over a schema: `users (id NOT NULL, name NOT NULL, secret NOT NULL
    /// INVISIBLE, email, bio)` and `posts (id NOT NULL, user_id NOT NULL, title NOT NULL, tag)`;
    /// each parameter with whether it may be null, and whether the columns were asked for.
    fn nullable(sql: &str) -> (Vec<(String, bool)>, bool) {
        let columns = json!({ "rows": [
            ["posts", "id", "NO", "auto_increment"], ["posts", "user_id", "NO", ""], ["posts", "title", "NO", ""], ["posts", "tag", "YES", ""],
            ["users", "id", "NO", "auto_increment"], ["users", "name", "NO", ""], ["users", "secret", "NO", "INVISIBLE"], ["users", "email", "YES", ""], ["users", "bio", "YES", ""],
        ] });
        let mut answers: HashMap<String, Answer> = HashMap::new();
        let mut asked = false;
        for _ in 0..8 {
            let host = Host::new(&answers);
            if let Some(analysis) = crate::mysql::analyze_text(sql, &host) {
                assert!(analysis.diagnostics.is_empty(), "{sql}: {:?}", analysis.diagnostics);
                return (analysis.params.iter().map(|p| (p.name.clone(), p.nullable)).collect(), asked);
            }
            for need in host.needs() {
                let (id, answer) = match need {
                    Need::Describe { id, sql: text } => {
                        let params: Vec<_> = text.matches('?').map(|_| json!({ "type": 253, "flags": 0, "charset": 255 })).collect();
                        (id, json!({ "mysql": { "columns": [], "params": params } }))
                    }
                    Need::Query { id, sql, .. } if sql == super::COLUMNS_QUERY => {
                        asked = true;
                        (id, columns.clone())
                    }
                    Need::Query { id, .. } => (id, json!({ "rows": [] })),
                };
                answers.insert(id, serde_json::from_value(answer).unwrap());
            }
        }
        panic!("the analysis never settled")
    }

    /// Each parameter's type, the prepare describing every `?` with `code` as MySQL 8.4 does for a context (8 an integer one, 16 a BIT one).
    fn param_types(sql: &str, code: u32) -> Vec<(String, String)> {
        let types = json!({ "rows": [
            ["t", "id", "int", "int", ""], ["t", "big", "bigint", "bigint unsigned", ""], ["t", "flag", "tinyint", "tinyint(1)", ""], ["t", "small", "tinyint", "tinyint", ""],
            ["t", "bit1", "bit", "bit(1)", ""], ["t", "bits", "bit", "bit(8)", ""], ["t", "hidden", "bigint", "bigint", "INVISIBLE"],
            ["u", "id", "bigint", "bigint", ""], ["u", "big", "int", "int", ""],
        ] });
        let mut answers: HashMap<String, Answer> = HashMap::new();
        for _ in 0..8 {
            let host = Host::new(&answers);
            if let Some(analysis) = crate::mysql::analyze_text(sql, &host) {
                assert!(analysis.diagnostics.is_empty(), "{sql}: {:?}", analysis.diagnostics);
                return analysis.params.iter().map(|p| (p.name.clone(), match &p.ty { crate::types::Type::Scalar { name } => name.clone(), other => format!("{other:?}") })).collect();
            }
            for need in host.needs() {
                let (id, answer) = match need {
                    Need::Describe { id, sql: text } => {
                        let params: Vec<_> = text.matches('?').map(|_| json!({ "type": code, "flags": 0, "charset": 63 })).collect();
                        (id, json!({ "mysql": { "columns": [], "params": params } }))
                    }
                    Need::Query { id, sql, .. } if sql == super::TYPES_QUERY => (id, types.clone()),
                    // The server's default mode, under which the engine reads `NOT` as the server does.
                    Need::Query { id, sql, .. } if sql == "select @@session.sql_mode" => (id, json!({ "rows": [["ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ENGINE_SUBSTITUTION"]] })),
                    Need::Query { id, .. } => (id, json!({ "rows": [] })),
                };
                answers.insert(id, serde_json::from_value(answer).unwrap());
            }
        }
        panic!("the analysis never settled")
    }

    #[test]
    fn a_parameter_written_to_or_compared_with_a_column_takes_the_values_its_rows_return() {
        let p = |name: &str, ty: &str| (name.to_string(), ty.to_string());
        // A BIGINT column's rows are BigInts, a TINYINT(1)'s and a BIT(1)'s truth values, a wider BIT's bytes.
        assert_eq!(param_types("select id from t where big = :b and flag = :f and id = :i and small = :s", 8), [p("b", "int8"), p("f", "bool|int4"), p("i", "int4"), p("s", "int4")]);
        assert_eq!(param_types("select id from t where bit1 = :one and bits = :eight", 16), [p("one", "bool|int4"), p("eight", "bytea")]);
        // Either side of a comparison, IN, BETWEEN, under NOT and OR, and a qualified column of a join.
        assert_eq!(param_types("select t.id from t join u on u.id = t.id where :b < t.big or not (u.id in (:x, :y)) and t.big between :lo and :hi", 8), [p("b", "int8"), p("x", "int8"), p("y", "int8"), p("lo", "int8"), p("hi", "int8")]);
        // A bare column of a join is ambiguous, and a column whose two uses disagree keeps the prepare's type.
        assert_eq!(param_types("select t.id from t join u on u.id = t.id where big = :b", 8), [p("b", "int4")]);
        assert_eq!(param_types("select t.id from t, u where t.big = :b and u.big = :b2 and t.id = :c and u.id = :c2", 8), [p("b", "int8"), p("b2", "int4"), p("c", "int4"), p("c2", "int8")]);
        // Written whole: VALUES by name and by place (the INVISIBLE column is no place), and an UPDATE's SET and WHERE.
        assert_eq!(param_types("insert into t (big, flag) values (:b, :f)", 8), [p("b", "int8"), p("f", "bool|int4")]);
        assert_eq!(param_types("insert into t values (:i, :b, :f, :s, :one, :eight)", 8), [p("i", "int4"), p("b", "int8"), p("f", "bool|int4"), p("s", "int4"), p("one", "bool|int4"), p("eight", "bytea")]);
        assert_eq!(param_types("update t set flag = :f where big = :b", 8), [p("f", "bool|int4"), p("b", "int8")]);
        assert_eq!(param_types("delete from t where big = :b", 8), [p("b", "int8")]);
        // An expression of the column is not the column: `big + 1` is the prepare's.
        assert_eq!(param_types("select id from t where big + 1 = :b limit :n", 8), [p("b", "int4"), p("n", "int4")]);
    }

    /// The tables `sql` reads and writes, as the freshness facts name them.
    fn tables(sql: &str) -> (Vec<String>, Vec<String>) {
        use sqlparser::dialect::MySqlDialect;
        use sqlparser::parser::Parser;
        let statement = Parser::parse_sql(&MySqlDialect {}, sql).unwrap().pop().unwrap();
        let mut t = super::tables(&statement);
        t.read.sort();
        t.read.dedup();
        (t.read, t.write)
    }

    #[test]
    fn the_facts_are_read_for_every_table_the_statement_names_and_no_with_query() {
        let names = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(tables("select u.id, (select count(*) from posts p where p.user_id = u.id) from `users` u where exists (select 1 from tags t where t.id = u.id)"), (names(&["posts", "tags", "users"]), vec![]));
        // A WITH query is no table, even one named like a table; a derived table's are read.
        assert_eq!(tables("with users as (select id from accounts) select id from users join (select user_id from posts) p on p.user_id = users.id"), (names(&["accounts", "posts"]), vec![]));
        // What an INSERT, an UPDATE and a DELETE write, and the tables they read.
        assert_eq!(tables("insert into audit (user_id) select id from users"), (names(&["users"]), names(&["audit"])));
        assert_eq!(tables("update users set name = (select max(title) from posts) where id = 1"), (names(&["posts", "users"]), names(&["users"])));
        assert_eq!(tables("delete from users where id in (select user_id from bans)"), (names(&["bans", "users"]), names(&["users"])));
        // A table of another schema is left out: the facts are read from the current one.
        assert_eq!(tables("select id from other.users"), (vec![], vec![]));
    }

    fn may_be_null(sql: &str) -> Vec<String> {
        nullable(sql).0.into_iter().filter(|(_, n)| *n).map(|(name, _)| name).collect()
    }

    #[test]
    fn a_parameter_written_whole_to_a_nullable_column_may_be_null() {
        assert_eq!(nullable("insert into users (name, email) values (:name, :email)"), (vec![("name".into(), false), ("email".into(), true)], true));
        assert_eq!(may_be_null("update users set email = :email, name = :name where id = :id"), ["email"]);
        assert_eq!(may_be_null("update users u set u.bio = :bio where u.id = :id"), ["bio"]);
        // Every row of a VALUES list, through a cast, which keeps a NULL; a REPLACE as an INSERT.
        assert_eq!(may_be_null("insert into users (name, email) values (:n1, :e1), (:n2, cast(:e2 as char))"), ["e1", "e2"]);
        assert_eq!(may_be_null("replace into users (name, email, bio) values (:name, :email, (:bio))"), ["email", "bio"]);
        // Without a column list, by position, past the INVISIBLE column, which takes no value.
        assert_eq!(may_be_null("insert into users values (:id, :name, :email, :bio)"), ["email", "bio"]);
        // The SET form, and ON DUPLICATE KEY UPDATE.
        assert_eq!(may_be_null("insert into users set name = :name, email = :email"), ["email"]);
        assert_eq!(may_be_null("insert into users (name, email) values (:name, :email) on duplicate key update bio = :bio, name = :again"), ["email", "bio"]);
        // A table of a join, by the name the statement gives it.
        assert_eq!(may_be_null("update users u join posts p on p.user_id = u.id set u.email = :email, p.title = :title, p.tag = :tag where p.id = :id"), ["email", "tag"]);
        // Not the whole value: the expression it is part of decides what is written.
        assert_eq!(nullable("update users set email = lower(:email) where id = :id"), (vec![("email".into(), false), ("id".into(), false)], false));
        assert!(may_be_null("insert into users (name, email) select name, :email from users").is_empty());
        // A read asks for no columns.
        assert_eq!(nullable("select id from users where email = :email"), (vec![("email".into(), false)], false));
    }

    #[test]
    fn a_parameter_the_statement_tests_for_null_or_coalesces_may_be_null() {
        assert_eq!(may_be_null("select id from users where coalesce(:n, 0) = 0 and ifnull(:m, 1) = 1 and isnull(:k) and :a <=> email and :b is not null and email = coalesce(email, :c)"), ["n", "m", "k", "a", "b"]);
        assert_eq!(may_be_null("select id from users where cast(:p as char) is null"), ["p"]);
        assert_eq!(may_be_null("update users set name = :name where coalesce(:id, id) = id"), ["id"]);
        assert_eq!(may_be_null("delete from users where :email is null"), ["email"]);
        // Through the column of a WITH query or a derived table that selects it, by its name, by
        // a FROM alias, by a column list, bare, and along a chain of WITH queries.
        let q = |sql: &str| may_be_null(sql);
        assert_eq!(q("with input as (select :q as q) select id from users, input where input.q is null or users.name = input.q"), ["q"]);
        assert_eq!(q("with input as (select :q as q) select id from users, input i where i.q is null"), ["q"]);
        assert_eq!(q("with input (v) as (select :q) select id from users, input where input.v is null"), ["q"]);
        assert_eq!(q("with input as (select :q as q) select id from users, input where q is null"), ["q"]);
        assert_eq!(q("with a as (select :q as q), b as (select a.q as r from a) select id from users, b where coalesce(b.r, 0) = 0"), ["q"]);
        assert_eq!(q("select id from users, (select cast(:q as signed) as q) as s where s.q is null or users.id = s.q"), ["q"]);
        assert_eq!(q("select id from users join (select :q as q) s on s.q is null"), ["q"]);
        assert_eq!(q("with input as (select :q as q) select id from users where exists (select 1 from input where input.q is null)"), ["q"]);
        // Not the parameter: another column of the item, a test of a table's column, a compound,
        // and a bare name a table between could hold.
        assert!(q("with input as (select :q as q, 1 as one) select id from users, input where input.one is null and users.email is null").is_empty());
        assert!(q("with input as (select :q as q union all select 1) select id from users, input where input.q is null").is_empty());
        assert!(q("with input as (select :q as q) select id from input where exists (select 1 from users where q is null)").is_empty());
        assert!(q("select id from users where name = :q").is_empty());
    }
}
