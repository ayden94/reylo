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
use sqlparser::ast::{self, DataType, Expr, FunctionArg, FunctionArgExpr, FunctionArguments, Query, SelectItem, SetExpr, TableWithJoins, Value};

/// How many synthesized statements one statement may cost; past it the statement's shapes are
/// abandoned rather than turning one prepare into dozens.
const MAX_PROBES: usize = 8;

#[derive(Debug, Default)]
pub struct Plan {
    /// Result column alias → its shape, in projection order.
    pub columns: Vec<(String, Shape)>,
    pub probes: Vec<Probe>,
}

/// Read one statement's JSON constructors; the probes come back as text for the build to prepare.
pub fn plan(q: &Query) -> Plan {
    let SetExpr::Select(select) = q.body.as_ref() else { return Plan::default() };
    let with = q.with.as_ref().map(|w| format!("{w} ")).unwrap_or_default();
    let mut b = Builder { stack: vec![(0, select.from.clone())], next_ctx: 0, drafts: Vec::new(), overflowed: false };
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
    Plan { columns, probes }
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

struct Builder {
    /// The `FROM` clauses in scope, outermost first, each with an id of its own so two sibling
    /// subqueries at one depth do not pool their columns into one probe.
    stack: Vec<(usize, Vec<TableWithJoins>)>,
    next_ctx: usize,
    drafts: Vec<Draft>,
    overflowed: bool,
}

struct Draft {
    ctx: usize,
    from: Vec<TableWithJoins>,
    /// The same `FROM` with every enclosing one appended, for a constructor whose `FROM` reads a
    /// correlated column; tried only when the narrow form fails to prepare.
    wider: Option<Vec<TableWithJoins>>,
    items: Vec<Expr>,
}

impl Builder {
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
                self.next_ctx += 1;
                self.stack.push((self.next_ctx, select.from.clone()));
                let shape = self.walk(inner);
                self.stack.pop();
                // A scalar subquery is one row only when it is an ungrouped aggregate; anything
                // else can match nothing and hand back NULL, and for a plain value the server's
                // own (nullable) verdict stands.
                if crate::mysql::exactly_one(q).is_ok() && crate::mysql::has_aggregate(inner) {
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
    f.name.0.last().map(|p| p.to_string().to_ascii_lowercase()).unwrap_or_default()
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
        let plan = plan(&q);
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
    fn subquery_items_are_planned_and_left_out_of_the_outer_probe() {
        let q = query("select u.id, u.name, (select count(*) from posts p where p.user_id = u.id) as n, (select p.id from posts p where p.user_id = u.id limit 1) as first from users u where u.id = ?");
        let plan = plan(&q);
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
    fn a_scalar_subquery_that_can_miss_is_nullable() {
        let q = query("select (select json_object('id', p.id) from posts p where p.user_id = u.id limit 1) as first, json_arrayagg(u.id) as ids, u.name from users u");
        let plan = plan(&q);
        assert!(matches!(plan.columns[0].1, Shape::Object { non_null: false, .. }));
        assert!(matches!(plan.columns[1].1, Shape::Array { non_null: false, .. }));
        assert_eq!(plan.columns.len(), 2);
    }
}
