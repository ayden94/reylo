//! MySQL's JSON constructors as nested shapes, the way [`crate::jsonshape`] reads Postgres's:
//! the constructor's value expressions are selected as real columns of a synthesized statement
//! over the constructor's own `FROM`, and the server's prepare types them, with the join's own
//! nullability. Recognised: `json_object(k, v, …)`, `json_arrayagg(x)`, `coalesce` / `ifnull`
//! guarding either with a non-null constant (`json_array()`, `'[]'`), `cast(… as json)`, and a
//! scalar subquery around any of them. A probe keeps the statement's `?` where they are: a
//! prepare binds nothing, and the server types a `?` from its context there too.
//!
//! The same probes repair one pessimism of the server's own flags (measured on 8.4): a correlated
//! ungrouped aggregate subquery in the select list marks every outer column and the subquery
//! itself nullable, while a GROUP BY, an EXISTS or a `limit 1` subquery leave them alone. So a
//! scalar subquery item is planned like a constructor (its item prepared over its own `FROM`,
//! kept only when the subquery is exactly one row), and [`outer_probe`] prepares the statement
//! without its subquery items; a column is NOT NULL when either prepare says so.

use crate::jsonshape::{Probe, Shape};
use sqlparser::ast::{self, DataType, Expr, FunctionArg, FunctionArgExpr, FunctionArguments, Query, SelectItem, SetExpr, TableFactor, TableWithJoins, Value};
use std::collections::{HashMap, HashSet};

/// The columns of the tables a statement names, by the table's name as the statement writes it,
/// each column in lower case, which is how MySQL compares a column name. What says whose an
/// aggregate in a scalar subquery is ([`Builder::owns`]).
pub type Columns = HashMap<String, HashSet<String>>;

/// How many synthesized statements one statement may cost; past it the statement's shapes are
/// abandoned rather than turning one prepare into dozens.
const MAX_PROBES: usize = 8;

#[derive(Debug, Default)]
pub struct Plan {
    /// Result column alias → its shape, in projection order.
    pub columns: Vec<(String, Shape)>,
    pub probes: Vec<Probe>,
    /// The leaves (probe, index) whose expression is a truth value: MySQL types a comparison as
    /// an integer, but writes it into a JSON document as `true` or `false`.
    pub booleans: Vec<(usize, usize)>,
    /// The tables whose columns the plan needed and `columns` did not have: with them it may
    /// read a subquery's aggregate over a bare column as the subquery's own. The probes do not
    /// depend on the answer.
    pub wanted: Vec<String>,
}

/// Read one statement's JSON constructors; the probes come back as text for the build to prepare.
pub fn plan(q: &Query, columns: &Columns) -> Plan {
    let SetExpr::Select(select) = q.body.as_ref() else { return Plan::default() };
    let with = q.with.as_ref().map(|w| format!("{w} ")).unwrap_or_default();
    let mut b = Builder { stack: vec![(0, select.from.clone())], columns, ctes: with_names(q), wanted: Vec::new(), next_ctx: 0, drafts: Vec::new(), overflowed: false, booleans: Vec::new() };
    let mut columns = Vec::new();
    for item in &select.projection {
        // The server names an unaliased expression by its text, which no build reproduces
        // reliably, so only an aliased constructor is matched to a result column.
        let SelectItem::ExprWithAlias { expr, alias } = item else { continue };
        if !is_constructor(expr) && !matches!(expr, Expr::Subquery(_)) {
            continue;
        }
        let before = b.drafts.len();
        let shape = b.walk(expr);
        if b.overflowed {
            b.drafts.truncate(before);
            break;
        }
        if shape != Shape::Unknown {
            columns.push((alias.value.clone(), shape));
        }
    }
    let probes = b.drafts.iter().map(|d| Probe { sql: text(&with, &d.items, &d.from), wider: d.wider.as_ref().map(|f| text(&with, &d.items, f)) }).collect();
    b.wanted.sort_unstable();
    b.wanted.dedup();
    Plan { columns, probes, booleans: b.booleans, wanted: b.wanted }
}

/// The names of a query's own WITH queries, in lower case.
fn with_names(q: &Query) -> Vec<String> {
    q.with.iter().flat_map(|w| w.cte_tables.iter().map(|c| c.alias.name.value.to_ascii_lowercase())).collect()
}

/// The statement without its subquery items, and the projection indexes it keeps: prepared, it
/// reports the outer columns' flags free of the subquery's pessimism. `None` when nothing would
/// change or a wildcard makes the positions unreadable.
pub fn outer_probe(q: &Query) -> Option<(String, Vec<usize>)> {
    let SetExpr::Select(select) = q.body.as_ref() else { return None };
    if select.projection.iter().any(|i| matches!(i, SelectItem::Wildcard(_) | SelectItem::QualifiedWildcard(..))) {
        return None;
    }
    let kept: Vec<usize> = select
        .projection
        .iter()
        .enumerate()
        .filter(|(_, item)| match item {
            SelectItem::UnnamedExpr(e) | SelectItem::ExprWithAlias { expr: e, .. } => !contains_subquery(e),
            _ => true,
        })
        .map(|(i, _)| i)
        .collect();
    if kept.is_empty() || kept.len() == select.projection.len() {
        return None;
    }
    let mut probe = (**select).clone();
    probe.projection = kept.iter().map(|&i| select.projection[i].clone()).collect();
    let with = q.with.as_ref().map(|w| format!("{w} ")).unwrap_or_default();
    Some((format!("{with}{probe}"), kept))
}

/// The columns an expression reads, each as its parts; false where the expression has a part this
/// does not read, a subquery among them, whose columns have a scope of their own.
fn references(e: &Expr, out: &mut Vec<Vec<String>>) -> bool {
    match e {
        Expr::Identifier(i) => {
            out.push(vec![i.value.clone()]);
            true
        }
        Expr::CompoundIdentifier(parts) => {
            out.push(parts.iter().map(|p| p.value.clone()).collect());
            true
        }
        Expr::Value(_) => true,
        Expr::Nested(inner) | Expr::UnaryOp { expr: inner, .. } | Expr::Cast { expr: inner, .. } | Expr::IsNull(inner) | Expr::IsNotNull(inner) => references(inner, out),
        Expr::BinaryOp { left, right, .. } => {
            let left = references(left, out);
            references(right, out) && left
        }
        Expr::Function(f) if f.over.is_none() && f.filter.is_none() && f.within_group.is_empty() => match &f.args {
            FunctionArguments::None => true,
            FunctionArguments::List(list) => {
                let mut whole = list.clauses.is_empty();
                for arg in &list.args {
                    whole &= match arg {
                        FunctionArg::Unnamed(FunctionArgExpr::Expr(x)) => references(x, out),
                        _ => false,
                    };
                }
                whole
            }
            FunctionArguments::Subquery(_) => false,
        },
        _ => false,
    }
}

/// Whether an expression holds a subquery anywhere the walk can see; what it cannot see counts as none, which only leaves that item in the probe.
fn contains_subquery(e: &Expr) -> bool {
    match e {
        Expr::Subquery(_) | Expr::Exists { .. } | Expr::InSubquery { .. } => true,
        Expr::Nested(inner) | Expr::UnaryOp { expr: inner, .. } | Expr::Cast { expr: inner, .. } | Expr::IsNull(inner) | Expr::IsNotNull(inner) => contains_subquery(inner),
        Expr::BinaryOp { left, right, .. } => contains_subquery(left) || contains_subquery(right),
        Expr::Function(f) => args(f).is_some_and(|a| a.iter().any(|x| contains_subquery(x))),
        Expr::Case { operand, conditions, else_result, .. } => {
            operand.as_deref().is_some_and(contains_subquery) || conditions.iter().any(|c| contains_subquery(&c.condition) || contains_subquery(&c.result)) || else_result.as_deref().is_some_and(contains_subquery)
        }
        _ => false,
    }
}

fn text(with: &str, items: &[Expr], from: &[TableWithJoins]) -> String {
    let items: Vec<String> = items.iter().map(|e| e.to_string()).collect();
    if from.is_empty() {
        format!("{with}SELECT {}", items.join(", "))
    } else {
        let from: Vec<String> = from.iter().map(|f| f.to_string()).collect();
        format!("{with}SELECT {} FROM {}", items.join(", "), from.join(", "))
    }
}

struct Builder<'a> {
    /// The `FROM` clauses in scope, outermost first, each with an id of its own so two sibling
    /// subqueries at one depth do not pool their columns into one probe.
    stack: Vec<(usize, Vec<TableWithJoins>)>,
    columns: &'a Columns,
    /// The WITH queries in scope where the builder stands, whose names are no tables'.
    ctes: Vec<String>,
    wanted: Vec<String>,
    next_ctx: usize,
    drafts: Vec<Draft>,
    overflowed: bool,
    booleans: Vec<(usize, usize)>,
}

struct Draft {
    ctx: usize,
    from: Vec<TableWithJoins>,
    /// The same `FROM` with every enclosing one appended, for a constructor whose `FROM` reads a
    /// correlated column; tried only when the narrow form fails to prepare.
    wider: Option<Vec<TableWithJoins>>,
    items: Vec<Expr>,
}

impl Builder<'_> {
    /// Whether an aggregate call in the select list of a subquery over `from` is that subquery's
    /// own, for certain. An aggregate belongs to the innermost query one of the columns it reads
    /// comes from, so one over columns of an outer query alone is that query's, and the subquery
    /// is then a row per row of its FROM: NULL as a scalar where that is empty. The call is the
    /// subquery's when it reads no column, a column qualified with the name of one of the FROM's
    /// tables, or a bare column one of those tables has (`columns`). Everything else proves
    /// nothing: a derived table, a WITH query, a table of another schema, an alias with a column
    /// list, a qualifier in another case than the alias, and a call with a part this does not
    /// read, unless it reads a column of the FROM beside it.
    fn owns(&mut self, f: &ast::Function, from: &[TableWithJoins]) -> bool {
        let mut reads: Vec<Vec<String>> = Vec::new();
        let mut whole = f.filter.is_none() && f.within_group.is_empty();
        match &f.args {
            FunctionArguments::None => {}
            FunctionArguments::List(list) => {
                whole &= list.clauses.is_empty();
                for arg in &list.args {
                    match arg {
                        FunctionArg::Unnamed(FunctionArgExpr::Expr(e)) => whole &= references(e, &mut reads),
                        // `count(*)` reads no column.
                        FunctionArg::Unnamed(FunctionArgExpr::Wildcard) => {}
                        _ => whole = false,
                    }
                }
            }
            FunctionArguments::Subquery(_) => whole = false,
        }
        // The FROM's tables: the name that qualifies each one's columns, and the table where its
        // columns are the catalog's to say.
        let mut items: Vec<(String, Option<String>)> = Vec::new();
        for factor in from.iter().flat_map(|t| std::iter::once(&t.relation).chain(t.joins.iter().map(|j| &j.relation))) {
            if let TableFactor::Table { name, alias, args: None, .. } = factor {
                let table = match name.0.as_slice() {
                    [ast::ObjectNamePart::Identifier(i)] => Some(i.value.clone()),
                    _ => None,
                };
                let known = table.clone().filter(|t| alias.as_ref().is_none_or(|a| a.columns.is_empty()) && !self.ctes.contains(&t.to_ascii_lowercase()));
                if let Some(qualifier) = alias.as_ref().map(|a| a.name.value.clone()).or(table) {
                    items.push((qualifier, known));
                }
            }
        }
        let mut own = false;
        for read in &reads {
            match read.as_slice() {
                [column] => {
                    for table in items.iter().filter_map(|(_, table)| table.as_ref()) {
                        match self.columns.get(table) {
                            Some(columns) => own |= columns.contains(&column.to_ascii_lowercase()),
                            None => self.wanted.push(table.clone()),
                        }
                    }
                }
                [qualifier, _] => own |= items.iter().any(|(name, _)| name == qualifier),
                _ => {}
            }
        }
        own || (whole && reads.is_empty())
    }

    fn walk(&mut self, e: &Expr) -> Shape {
        if self.overflowed {
            return Shape::Unknown;
        }
        match e {
            Expr::Nested(inner) => self.walk(inner),
            Expr::Function(f) => {
                let Some(args) = args(f) else { return self.leaf(e) };
                match name(f).as_str() {
                    "json_object" => {
                        if args.len() % 2 != 0 {
                            return Shape::Unknown;
                        }
                        let mut fields = Vec::with_capacity(args.len() / 2);
                        for pair in args.chunks(2) {
                            let Some(key) = string_const(pair[0]) else { return Shape::Unknown };
                            fields.push((key, self.walk(pair[1])));
                        }
                        Shape::Object { fields, non_null: true }
                    }
                    // An aggregate over no rows is NULL, so the array is non-null only under a guard.
                    "json_arrayagg" => {
                        if args.len() != 1 {
                            return Shape::Unknown;
                        }
                        Shape::Array { items: Box::new(self.walk(args[0])), non_null: false }
                    }
                    "coalesce" | "ifnull" => {
                        let Some(first) = args.first() else { return Shape::Unknown };
                        let guarded = args[1..].iter().any(|a| is_non_null_const(a));
                        match (self.walk(first), guarded) {
                            (Shape::Object { fields, .. }, true) => Shape::Object { fields, non_null: true },
                            (Shape::Array { items, .. }, true) => Shape::Array { items, non_null: true },
                            (shape, _) => shape,
                        }
                    }
                    _ => self.leaf(e),
                }
            }
            Expr::Cast { expr, data_type: DataType::JSON, .. } => self.walk(expr),
            Expr::Subquery(q) => {
                let SetExpr::Select(select) = q.body.as_ref() else { return Shape::Unknown };
                let Some(inner) = single_projection(q) else { return Shape::Unknown };
                let outer_ctes = self.ctes.len();
                self.ctes.extend(with_names(q));
                self.next_ctx += 1;
                self.stack.push((self.next_ctx, select.from.clone()));
                let shape = self.walk(inner);
                self.stack.pop();
                // A scalar subquery is one row only when it is an ungrouped aggregate of its own
                // (`owns`); anything else can match nothing and hand back NULL, and for a plain
                // value the server's own (nullable) verdict stands. A LIMIT is left out here, as
                // it was before the row count learned that a constant one of a row or more keeps
                // the row.
                let one_row = q.limit_clause.is_none() && crate::mysql::exactly_one(q).is_ok() && crate::mysql::aggregate_where(inner, &mut |f| self.owns(f, &select.from));
                self.ctes.truncate(outer_ctes);
                if one_row {
                    shape
                } else {
                    match shape {
                        Shape::Column { .. } => Shape::Unknown,
                        other => force_nullable(other),
                    }
                }
            }
            _ => self.leaf(e),
        }
    }

    /// One value expression, as a column of the probe for the constructor's own `FROM` context.
    fn leaf(&mut self, e: &Expr) -> Shape {
        let ctx = self.stack[self.stack.len() - 1].0;
        let probe = match self.drafts.iter().position(|d| d.ctx == ctx) {
            Some(i) => i,
            None => match self.draft() {
                Some(i) => i,
                None => return Shape::Unknown,
            },
        };
        let index = self.drafts[probe].items.len();
        self.drafts[probe].items.push(e.clone());
        if is_truth_value(e) {
            self.booleans.push((probe, index));
        }
        Shape::Column { probe, index }
    }

    fn draft(&mut self) -> Option<usize> {
        if self.drafts.len() >= MAX_PROBES {
            self.overflowed = true;
            return None;
        }
        let depth = self.stack.len() - 1;
        let (ctx, from) = self.stack[depth].clone();
        let mut wider = from.clone();
        for d in (0..depth).rev() {
            wider.extend(self.stack[d].1.iter().cloned());
        }
        // A subquery with no `FROM` of its own only ever resolves against the enclosing one.
        let (from, wider) = if from.is_empty() && !wider.is_empty() { (wider, None) } else { (from, Some(wider)) };
        let wider = wider.filter(|w| w.len() != from.len());
        self.drafts.push(Draft { ctx, from, wider, items: Vec::new() });
        Some(self.drafts.len() - 1)
    }
}

fn force_nullable(shape: Shape) -> Shape {
    match shape {
        Shape::Object { fields, .. } => Shape::Object { fields, non_null: false },
        Shape::Array { items, .. } => Shape::Array { items, non_null: false },
        other => other,
    }
}

/// Whether an expression is one the shapes speak for, down to a constructor at its core.
pub fn is_constructor(e: &Expr) -> bool {
    match e {
        Expr::Nested(inner) => is_constructor(inner),
        Expr::Function(f) => match name(f).as_str() {
            "json_object" | "json_arrayagg" => true,
            "coalesce" | "ifnull" => args(f).and_then(|a| a.first().copied()).is_some_and(is_constructor),
            _ => false,
        },
        Expr::Cast { expr, data_type: DataType::JSON, .. } => is_constructor(expr),
        Expr::Subquery(q) => single_projection(q).is_some_and(is_constructor),
        _ => false,
    }
}

fn name(f: &ast::Function) -> String {
    crate::mysql::builtin(f)
}

/// The plain expression arguments of a call; `None` for a wildcard, a named argument or a subquery form.
fn args(f: &ast::Function) -> Option<Vec<&Expr>> {
    match &f.args {
        FunctionArguments::None => Some(Vec::new()),
        FunctionArguments::List(list) => list
            .args
            .iter()
            .map(|a| match a {
                FunctionArg::Unnamed(FunctionArgExpr::Expr(e)) => Some(e),
                _ => None,
            })
            .collect(),
        FunctionArguments::Subquery(_) => None,
    }
}

fn single_projection(q: &Query) -> Option<&Expr> {
    let SetExpr::Select(select) = q.body.as_ref() else { return None };
    match select.projection.as_slice() {
        [SelectItem::UnnamedExpr(e)] | [SelectItem::ExprWithAlias { expr: e, .. }] => Some(e),
        _ => None,
    }
}

fn string_const(e: &Expr) -> Option<String> {
    match e {
        Expr::Value(v) => match &v.value {
            Value::SingleQuotedString(s) | Value::DoubleQuotedString(s) => Some(s.clone()),
            _ => None,
        },
        Expr::Nested(inner) => string_const(inner),
        _ => None,
    }
}

/// A constant that is never NULL, so a `coalesce` over it is never NULL: a literal, or an empty
/// `json_array()` / `json_object()`.
/// Whether an expression is a truth value, which a JSON constructor writes as `true` or `false`.
fn is_truth_value(e: &Expr) -> bool {
    use ast::{BinaryOperator as B, UnaryOperator as U};
    match e {
        Expr::Nested(inner) => is_truth_value(inner),
        Expr::Value(v) => matches!(v.value, Value::Boolean(_)),
        Expr::BinaryOp { op, .. } => matches!(op, B::Eq | B::NotEq | B::Lt | B::LtEq | B::Gt | B::GtEq | B::Spaceship | B::And | B::Or | B::Xor),
        Expr::UnaryOp { op: U::Not, .. } => true,
        Expr::IsNull(_) | Expr::IsNotNull(_) | Expr::IsTrue(_) | Expr::IsNotTrue(_) | Expr::IsFalse(_) | Expr::IsNotFalse(_) | Expr::IsUnknown(_) | Expr::IsNotUnknown(_) => true,
        Expr::InList { .. } | Expr::InSubquery { .. } | Expr::Between { .. } | Expr::Like { .. } | Expr::RLike { .. } | Expr::Exists { .. } => true,
        _ => false,
    }
}

fn is_non_null_const(e: &Expr) -> bool {
    match e {
        Expr::Value(v) => !matches!(v.value, Value::Null | Value::Placeholder(_)),
        Expr::Nested(inner) | Expr::Cast { expr: inner, .. } => is_non_null_const(inner),
        Expr::Function(f) => matches!(name(f).as_str(), "json_array" | "json_object"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::MySqlDialect;
    use sqlparser::parser::Parser;

    fn query(sql: &str) -> Query {
        let mut v = Parser::parse_sql(&MySqlDialect {}, sql).unwrap();
        let Statement::Query(q) = v.pop().unwrap() else { panic!("not a query") };
        *q
    }

    #[test]
    fn nested_fetch_plans_two_probes() {
        let q = query("select u.id, (select coalesce(json_arrayagg(json_object('id', p.id, 'title', p.title)), json_array()) from posts p where p.user_id = u.id) as posts, json_object('email', u.email, 'n', u.id + ?) as meta from users u where u.id = ?");
        let plan = plan(&q, &Columns::new());
        assert_eq!(plan.probes.len(), 2);
        assert_eq!(plan.probes[0].sql, "SELECT p.id, p.title FROM posts p");
        assert_eq!(plan.probes[0].wider.as_deref(), Some("SELECT p.id, p.title FROM posts p, users u"));
        assert_eq!(plan.probes[1].sql, "SELECT u.email, u.id + ? FROM users u");
        assert_eq!(plan.probes[1].wider, None);
        let posts = Shape::Array { items: Box::new(Shape::Object { fields: vec![("id".into(), Shape::Column { probe: 0, index: 0 }), ("title".into(), Shape::Column { probe: 0, index: 1 })], non_null: true }), non_null: true };
        let meta = Shape::Object { fields: vec![("email".into(), Shape::Column { probe: 1, index: 0 }), ("n".into(), Shape::Column { probe: 1, index: 1 })], non_null: true };
        assert_eq!(plan.columns, vec![("posts".to_string(), posts), ("meta".to_string(), meta)]);
    }

    #[test]
    fn a_truth_value_inside_a_constructor_is_marked_boolean() {
        let q = query("select json_object('t', true, 'c', u.id > 0, 'in', u.id in (1, 2), 'not', not (u.id = 1), 'n', u.id, 's', u.name) as o from users u");
        let plan = plan(&q, &Columns::new());
        assert_eq!(plan.probes[0].sql, "SELECT true, u.id > 0, u.id IN (1, 2), NOT (u.id = 1), u.id, u.name FROM users u");
        assert_eq!(plan.booleans, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
    }

    #[test]
    fn subquery_items_are_planned_and_left_out_of_the_outer_probe() {
        let q = query("select u.id, u.name, (select count(*) from posts p where p.user_id = u.id) as n, (select p.id from posts p where p.user_id = u.id limit 1) as first from users u where u.id = ?");
        let plan = plan(&q, &Columns::new());
        // The exactly-one aggregate keeps its column; the `limit 1` fetch falls back to the server's verdict.
        assert_eq!(plan.columns, vec![("n".to_string(), Shape::Column { probe: 0, index: 0 })]);
        assert_eq!(plan.probes[0].sql, "SELECT count(*) FROM posts p");
        let (sql, kept) = outer_probe(&q).unwrap();
        assert_eq!(sql, "SELECT u.id, u.name FROM users u WHERE u.id = ?");
        assert_eq!(kept, vec![0, 1]);
        assert!(outer_probe(&query("select u.id, u.name from users u")).is_none());
        assert!(outer_probe(&query("select u.*, (select count(*) from posts) as n from users u")).is_none());
    }

    #[test]
    fn an_aggregate_over_outer_columns_leaves_the_subquery_nullable() {
        let columns: Columns = [("posts", vec!["id", "user_id", "title"]), ("users", vec!["id", "name"])].into_iter().map(|(t, c)| (t.to_string(), c.into_iter().map(String::from).collect())).collect();
        // The first column's shape, and the tables whose columns the plan lacked.
        let first = |sql: &str, columns: &Columns| {
            let plan = plan(&query(sql), columns);
            (plan.columns.first().map(|(_, shape)| shape.clone()), plan.wanted)
        };
        let kept = |sql: &str| matches!(first(sql, &columns).0, Some(Shape::Column { .. } | Shape::Array { non_null: true, .. }));
        // The subquery's own: no column, a column of its FROM by qualifier, a bare one a table of
        // its FROM has, and any of those beside an outer column.
        for sql in [
            "select (select count(*) from posts p where p.user_id = u.id) as n from users u",
            "select (select count(p.id) from posts p where p.user_id = u.id) as n from users u",
            "select (select count(posts.id) from posts) as n from users u",
            "select (select count(title) from posts where user_id = u.id) as n from users u",
            "select (select coalesce(sum(length(title)), 0) from posts) as n from users u",
            "select (select count(distinct title) from posts) as n from users u",
            "select (select count(length(name) + length(title)) from posts) as n from users u",
            "select (select coalesce(json_arrayagg(title), json_array()) from posts) as titles from users u",
            "select (select group_concat(title order by name) from posts) as n from users u",
        ] {
            assert!(kept(sql), "{sql}");
        }
        // The outer query's, or not provably the subquery's.
        for sql in [
            "select (select count(u.id) from posts) as n from users u",
            "select (select count(name) from posts) as n from users u",
            "select (select coalesce(max(length(name)), 0) from posts) as n from users u",
            "select (select coalesce(json_arrayagg(name), json_array()) from posts) as names from users u",
            "select (select coalesce(json_arrayagg(u.name), json_array()) from posts) as names from users u",
            // The qualifier is another name than the alias, in case too.
            "select (select count(posts.id) from posts p) as n from users posts",
            "select (select count(P.id) from posts p) as n from users P",
            // A WITH query, a derived table and a table of another schema do not say their columns here.
            "with posts as (select id from users) select (select count(title) from posts) as n from users u",
            "select (select count(title) from (select title from posts) p) as n from users u",
            "select (select count(title) from app.posts) as n from users u",
            "select (select count(title) from posts p (a, b, c)) as n from users u",
            // A part of the call this does not read.
            "select (select count(case when title is null then 1 end) from posts) as n from users u",
            "select (select group_concat(name order by title) from posts) as n from users u",
        ] {
            assert!(!kept(sql), "{sql}");
        }
        // A constructor qualified with a schema is a stored function of that name.
        assert!(plan(&query("select app.json_object('id', u.id) as o from users u"), &columns).columns.is_empty());
        assert!(!kept("select (select app.count(p.id) from posts p) as n from users u"));
        // Without the table's columns a bare name proves nothing, and the plan says which table.
        let bare = "select (select count(title) from posts where user_id = u.id) as n from users u";
        assert_eq!(first(bare, &Columns::new()), (None, vec!["posts".to_string()]));
        assert_eq!(first(bare, &columns).1, Vec::<String>::new());
        assert_eq!(first("select (select count(p.id) from posts p) as n from users u", &Columns::new()).1, Vec::<String>::new());
    }

    #[test]
    fn a_scalar_subquery_that_can_miss_is_nullable() {
        let q = query("select (select json_object('id', p.id) from posts p where p.user_id = u.id limit 1) as first, json_arrayagg(u.id) as ids, u.name from users u");
        let plan = plan(&q, &Columns::new());
        assert!(matches!(plan.columns[0].1, Shape::Object { non_null: false, .. }));
        assert!(matches!(plan.columns[1].1, Shape::Array { non_null: false, .. }));
        assert_eq!(plan.columns.len(), 2);
    }
}
