//! Nested JSON shapes, recovered from the SQL that builds them.
//!
//! A column built with `jsonb_build_object(…)` or `jsonb_agg(…)` describes as bare `jsonb` and
//! nothing more: the server reports the constructor's return type, never the shape it
//! constructed. The shape is not lost, though — the constructor's arguments are ordinary SQL
//! expressions in an ordinary `FROM` context, and the database can type those. So the build
//! reads the constructor out of the parse tree, synthesizes a statement that selects those
//! expressions as real columns, and describes it through the path every other statement takes.
//! The nested fields then get type oids with table-and-attribute provenance, which means the
//! same catalog `NOT NULL` and the same outer-join degradation apply inside the nested shape.
//!
//! Recognised: `json_build_object` / `jsonb_build_object`, `json_agg` / `jsonb_agg`,
//! `coalesce(json[b]_agg(…), '[]')` — which is what makes a nested array non-null —
//! `to_json` / `to_jsonb` / `row_to_json` over a whole row, and any nesting of those. SQLite
//! spells the first two `json_object` and `json_group_array`, which are recognised too; it has
//! no row constructor. Everything else is JSON the build cannot explain — a stored column, a
//! `||`, a function's return — and nothing is claimed for it.
//!
//! This module is pure: `plan` reads the parse tree and synthesizes statement text, `resolve`
//! turns the describes of that text into JSON Schema. The build owns the round trips.

use crate::pgtypes::{oid, TypeRegistry};
use crate::types::{DescribedColumn, Field, Type};
use pg_query::protobuf::{self, node::Node as NodeEnum, AStar, ColumnRef, Node, ParseResult, RawStmt, ResTarget, SelectStmt};
use serde_json::Value;

/// `json_build_object(k, v, …)` and its jsonb twin: the main case. `json_object` is SQLite's
/// spelling of it; Postgres's own `json_object` takes arrays, whose arguments are never the
/// literal keys this reads, so it falls through unrecognised there.
const OBJECT_FNS: &[&str] = &["json_build_object", "jsonb_build_object", "json_object"];
/// `json_agg(x)` and its jsonb twin: the same shape, as an array. `json_group_array` is
/// SQLite's, and it answers `[]` rather than NULL over an empty group.
const AGG_FNS: &[&str] = &["json_agg", "jsonb_agg", "json_group_array"];
/// The aggregates that are never NULL, so a nested array under one needs no `coalesce`.
const TOTAL_AGG_FNS: &[&str] = &["json_group_array"];
/// A whole row, so the shape is the relation's own columns.
const ROW_FNS: &[&str] = &["to_json", "to_jsonb", "row_to_json"];

/// How many synthesized statements one statement may cost. A nested fetch needs one; a shape
/// with an aggregate under an object needs two. The cap keeps a pathological statement from
/// turning one describe into dozens.
const MAX_PROBES: usize = 8;

/// A value the build recovered from a JSON constructor.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// `json[b]_build_object(k, v, …)`, with the keys in the order the SQL writes them. The
    /// constructor never returns SQL NULL, so `non_null` is true unless something above it —
    /// a scalar subquery that may match no row — can null the whole value.
    Object { fields: Vec<(String, Shape)>, non_null: bool },
    /// `json[b]_agg(x)`. An aggregate over no rows is NULL, so `non_null` is true only under a
    /// `coalesce(…, '[]'::jsonb)`.
    Array { items: Box<Shape>, non_null: bool },
    /// One value expression: column `index` of probe `probe`.
    Column { probe: usize, index: usize },
    /// A whole row: every column of probe `probe`, which selects `alias.*`.
    Row { probe: usize },
    /// A bound parameter, `$n`; its type is the one the statement's own describe reported.
    Param(i32),
    /// Nothing the build derived.
    Unknown,
}

/// A synthesized statement whose result columns are one constructor's value expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct Probe {
    /// Over the constructor's own `FROM` clause.
    pub sql: String,
    /// The same target list over that `FROM` and every enclosing one, for a constructor that
    /// reads a correlated column the inner `FROM` alone cannot resolve. It is tried only when
    /// the narrow form fails, so a plain nested fetch costs exactly one extra describe.
    pub wider: Option<String>,
}

/// What one statement's JSON columns need: a shape per result column the build could explain,
/// and the statements to describe to fill it in.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    /// Result column name → its shape, in target-list order.
    pub columns: Vec<(String, Shape)>,
    pub probes: Vec<Probe>,
}

/// One probe's describe, as the resolver needs it.
#[derive(Debug, Clone)]
pub struct Outcome {
    pub columns: Vec<DescribedColumn>,
    /// Whether the probe's own analysis saw an outer join, which is the one thing that can null
    /// a whole row under `to_jsonb(alias)`.
    pub outer_join: bool,
}

/// A resolved shape: the column's type, whether the column can be SQL NULL, and the paths
/// inside it whose type the engine could not derive.
#[derive(Debug, Clone)]
pub struct Resolved {
    pub ty: Type,
    pub non_null: bool,
    pub unresolved: Vec<String>,
}

/// Read one rewritten statement's JSON constructors. Nothing runs against the database here;
/// the probes come back as text for the build to describe.
pub fn plan(sql: &str) -> Plan {
    let Ok(parsed) = pg_query::parse(sql) else { return Plan::default() };
    let version = parsed.protobuf.version;
    let Some(top) = parsed.protobuf.stmts.first().and_then(|s| s.stmt.as_deref()).and_then(|n| n.node.as_ref()) else {
        return Plan::default();
    };
    let Some((targets, from, with)) = top_context(top) else { return Plan::default() };
    let mut builder = Builder { stack: vec![(0, from)], next_ctx: 0, drafts: Vec::new(), overflowed: false };
    let mut columns: Vec<(String, Shape)> = Vec::new();
    for target in &targets {
        let Some(NodeEnum::ResTarget(rt)) = target.node.as_ref() else { continue };
        let Some(val) = rt.val.as_deref() else { continue };
        if !is_constructor(val) {
            continue;
        }
        let Some(name) = target_name(rt) else { continue };
        let before = builder.drafts.len();
        let shape = builder.walk(val);
        if builder.overflowed {
            builder.drafts.truncate(before);
            break;
        }
        if shape != Shape::Unknown {
            columns.push((name, shape));
        }
    }
    // Every probe carries the statement's own CTEs, so a constructor reading one still
    // resolves; a CTE holding a placeholder is left out, because a probe binds nothing.
    let with = with.filter(|w| !contains_param(w));
    let probes = builder
        .drafts
        .iter()
        .map(|d| Probe {
            sql: select_text(version, &d.items, &d.from, with.as_ref()).unwrap_or_default(),
            wider: d.wider.as_ref().and_then(|f| select_text(version, &d.items, f, with.as_ref())),
        })
        .collect();
    Plan { columns, probes }
}

/// Turn one shape plus its probes' describes into the column's type.
pub fn resolve(shape: &Shape, probes: &[Option<Outcome>], registry: &TypeRegistry, param_oids: &[u32]) -> Resolved {
    let mut unresolved = Vec::new();
    let ctx = ResolveCtx { probes, registry, param_oids };
    let (ty, non_null) = ctx.node(shape, "", &mut unresolved);
    Resolved { ty, non_null, unresolved }
}

/// Every oid a resolved shape reads, so the host can be asked for the ones the registry lacks.
pub fn oids(shape: &Shape, probes: &[Option<Outcome>], param_oids: &[u32], out: &mut Vec<u32>) {
    match shape {
        Shape::Object { fields, .. } => fields.iter().for_each(|(_, s)| oids(s, probes, param_oids, out)),
        Shape::Array { items, .. } => oids(items, probes, param_oids, out),
        Shape::Column { probe, index } => {
            if let Some(col) = probes.get(*probe).and_then(|o| o.as_ref()).and_then(|o| o.columns.get(*index)) {
                out.push(col.type_oid);
            }
        }
        Shape::Row { probe } => {
            if let Some(o) = probes.get(*probe).and_then(|o| o.as_ref()) {
                out.extend(o.columns.iter().map(|c| c.type_oid));
            }
        }
        Shape::Param(n) => out.extend(param_oids.get((*n as usize).saturating_sub(1)).copied()),
        Shape::Unknown => {}
    }
}

/// Whether a type is one the shape machinery speaks for: a JSON column and nothing else.
pub fn is_json(registry: &TypeRegistry, type_oid: u32) -> bool {
    matches!(registry.resolve_domain(type_oid), oid::JSON | oid::JSONB)
}

// ---------------------------------------------------------------------------------------------
// Recognition
// ---------------------------------------------------------------------------------------------

struct Builder {
    /// The `FROM` clauses in scope, outermost first; the innermost is the last. Each carries an
    /// id of its own, because two sibling subqueries sit at the same depth and select from
    /// different relations — keying a probe on the depth alone would pool their columns.
    stack: Vec<(usize, Vec<Node>)>,
    next_ctx: usize,
    drafts: Vec<Draft>,
    /// Set once the probe budget is spent, which abandons the statement's inference.
    overflowed: bool,
}

struct Draft {
    ctx: usize,
    /// A `to_jsonb(alias)` probe, whose every column is one field.
    row: bool,
    from: Vec<Node>,
    /// The same `FROM` with every enclosing one appended, for a correlated constructor.
    wider: Option<Vec<Node>>,
    items: Vec<Node>,
}

impl Builder {
    fn walk(&mut self, expr: &Node) -> Shape {
        if self.overflowed {
            return Shape::Unknown;
        }
        match expr.node.as_ref() {
            Some(NodeEnum::FuncCall(f)) => {
                let name = func_name(f).unwrap_or_default();
                if OBJECT_FNS.contains(&name.as_str()) {
                    if f.args.len() % 2 != 0 {
                        return Shape::Unknown;
                    }
                    let mut fields = Vec::with_capacity(f.args.len() / 2);
                    for pair in f.args.chunks(2) {
                        let Some(key) = string_const(&pair[0]) else { return Shape::Unknown };
                        fields.push((key, self.walk(&pair[1])));
                    }
                    Shape::Object { fields, non_null: true }
                } else if AGG_FNS.contains(&name.as_str()) {
                    if f.args.len() != 1 {
                        return Shape::Unknown;
                    }
                    let non_null = TOTAL_AGG_FNS.contains(&name.as_str());
                    Shape::Array { items: Box::new(self.walk(&f.args[0])), non_null }
                } else if ROW_FNS.contains(&name.as_str()) {
                    match f.args.first().and_then(bare_alias) {
                        Some(alias) if f.args.len() == 1 => self.row(&alias),
                        _ => Shape::Unknown,
                    }
                } else {
                    self.leaf(expr)
                }
            }
            Some(NodeEnum::CoalesceExpr(c)) => {
                let Some(first) = c.args.first() else { return Shape::Unknown };
                let guarded = c.args[1..].iter().any(is_non_null_const);
                match (self.walk(first), guarded) {
                    (Shape::Object { fields, .. }, true) => Shape::Object { fields, non_null: true },
                    (Shape::Array { items, .. }, true) => Shape::Array { items, non_null: true },
                    (shape, _) => shape,
                }
            }
            Some(NodeEnum::TypeCast(t)) => match (t.arg.as_deref(), casts_to_json(t)) {
                (Some(arg), true) => self.walk(arg),
                _ => self.leaf(expr),
            },
            Some(NodeEnum::SubLink(s)) => {
                if s.sub_link_type != protobuf::SubLinkType::ExprSublink as i32 {
                    return Shape::Unknown;
                }
                let Some(select) = s.subselect.as_deref().and_then(as_select) else { return Shape::Unknown };
                let Some(inner) = single_target(select).and_then(|rt| rt.val.clone()) else { return Shape::Unknown };
                self.next_ctx += 1;
                self.stack.push((self.next_ctx, select.from_clause.clone()));
                let shape = self.walk(&inner);
                self.stack.pop();
                // A scalar subquery is one row only when it is an ungrouped aggregate; anything
                // else can match nothing and hand back NULL.
                if crate::infer::single_row(select) && aggregates(&inner) {
                    shape
                } else {
                    force_nullable(shape)
                }
            }
            _ => self.leaf(expr),
        }
    }

    /// One value expression, as a column of the probe for the constructor's own `FROM` context.
    fn leaf(&mut self, expr: &Node) -> Shape {
        if let Some(n) = param_number(expr) {
            return Shape::Param(n);
        }
        // A probe binds nothing, so an expression reading a placeholder cannot go in one.
        if contains_param(expr) {
            return Shape::Unknown;
        }
        let ctx = self.stack[self.stack.len() - 1].0;
        let probe = match self.drafts.iter().position(|d| d.ctx == ctx && !d.row) {
            Some(i) => i,
            None => match self.draft(false) {
                Some(i) => i,
                None => return Shape::Unknown,
            },
        };
        let index = self.drafts[probe].items.len();
        self.drafts[probe].items.push(expr.clone());
        Shape::Column { probe, index }
    }

    /// `to_jsonb(u)`: a probe of its own, selecting `u.*`, so the row's own columns are the shape.
    fn row(&mut self, alias: &str) -> Shape {
        let Some(probe) = self.draft(true) else { return Shape::Unknown };
        let star = Node {
            node: Some(NodeEnum::ColumnRef(ColumnRef {
                fields: vec![string_node(alias), Node { node: Some(NodeEnum::AStar(AStar {})) }],
                location: -1,
            })),
        };
        self.drafts[probe].items.push(star);
        Shape::Row { probe }
    }

    fn draft(&mut self, row: bool) -> Option<usize> {
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
        self.drafts.push(Draft { ctx, row, from, wider, items: Vec::new() });
        Some(self.drafts.len() - 1)
    }
}

/// The target list, `FROM` context, and `WITH` clause the top statement's result columns live in.
pub(crate) fn top_context(top: &NodeEnum) -> Option<(Vec<Node>, Vec<Node>, Option<protobuf::WithClause>)> {
    match top {
        NodeEnum::SelectStmt(s) => {
            // A set operation hides which arm produced a column; the build degrades there anyway.
            if s.op != protobuf::SetOperation::SetopNone as i32 && s.op != 0 {
                return None;
            }
            Some((s.target_list.clone(), s.from_clause.clone(), s.with_clause.clone()))
        }
        NodeEnum::InsertStmt(s) => {
            Some((s.returning_list.clone(), relation_from(s.relation.as_ref(), &[]), s.with_clause.clone()))
        }
        NodeEnum::UpdateStmt(s) => {
            Some((s.returning_list.clone(), relation_from(s.relation.as_ref(), &s.from_clause), s.with_clause.clone()))
        }
        NodeEnum::DeleteStmt(s) => {
            Some((s.returning_list.clone(), relation_from(s.relation.as_ref(), &s.using_clause), s.with_clause.clone()))
        }
        _ => None,
    }
}

/// A DML statement's `RETURNING` reads the target relation plus whatever it joined in.
fn relation_from(relation: Option<&protobuf::RangeVar>, extra: &[Node]) -> Vec<Node> {
    let mut out = Vec::new();
    if let Some(r) = relation {
        out.push(Node { node: Some(NodeEnum::RangeVar(r.clone())) });
    }
    out.extend(extra.iter().cloned());
    out
}

/// Whether this expression is a JSON constructor the build speaks for, rather than an ordinary
/// expression that happens to be typed json.
fn is_constructor(expr: &Node) -> bool {
    match expr.node.as_ref() {
        Some(NodeEnum::FuncCall(f)) => {
            let name = func_name(f).unwrap_or_default();
            OBJECT_FNS.contains(&name.as_str()) || AGG_FNS.contains(&name.as_str()) || ROW_FNS.contains(&name.as_str())
        }
        Some(NodeEnum::CoalesceExpr(c)) => c.args.first().is_some_and(is_constructor),
        Some(NodeEnum::TypeCast(t)) => casts_to_json(t) && t.arg.as_deref().is_some_and(is_constructor),
        Some(NodeEnum::SubLink(s)) => {
            s.sub_link_type == protobuf::SubLinkType::ExprSublink as i32
                && s.subselect
                    .as_deref()
                    .and_then(as_select)
                    .and_then(single_target)
                    .and_then(|rt| rt.val.clone())
                    .is_some_and(|v| is_constructor(&v))
        }
        _ => false,
    }
}

/// Whether the top of this expression is an aggregate, which is what makes an ungrouped
/// subquery return exactly one row. A call with `OVER` is a window function, one row per input.
fn aggregates(expr: &Node) -> bool {
    match expr.node.as_ref() {
        Some(NodeEnum::FuncCall(f)) => f.over.is_none() && func_name(f).is_some_and(|n| AGG_FNS.contains(&n.as_str())),
        Some(NodeEnum::CoalesceExpr(c)) => c.args.first().is_some_and(aggregates),
        Some(NodeEnum::TypeCast(t)) => t.arg.as_deref().is_some_and(aggregates),
        _ => false,
    }
}

fn force_nullable(shape: Shape) -> Shape {
    match shape {
        Shape::Object { fields, .. } => Shape::Object { fields, non_null: false },
        Shape::Array { items, .. } => Shape::Array { items, non_null: false },
        other => other,
    }
}

// ---------------------------------------------------------------------------------------------
// Synthesis
// ---------------------------------------------------------------------------------------------

/// `SELECT <items> FROM <from>`, deparsed by the same library that parsed the original. The
/// statement is only ever prepared, never executed, so it needs no `WHERE` to stay cheap.
pub(crate) fn select_text(version: i32, items: &[Node], from: &[Node], with: Option<&protobuf::WithClause>) -> Option<String> {
    if items.is_empty() {
        return None;
    }
    let target_list = items
        .iter()
        .enumerate()
        .map(|(i, val)| Node {
            node: Some(NodeEnum::ResTarget(Box::new(ResTarget {
                // A star cannot take an alias; everything else gets a positional one, since the
                // shape maps back by position.
                name: if is_star(val) { String::new() } else { format!("a{i}") },
                indirection: Vec::new(),
                val: Some(Box::new(val.clone())),
                location: -1,
            }))),
        })
        .collect();
    // Every enumerated field has to carry a value the deparser knows: a protobuf zero means
    // "unset", and libpg_query asserts rather than guessing.
    let select = SelectStmt {
        target_list,
        from_clause: from.to_vec(),
        with_clause: with.cloned(),
        op: protobuf::SetOperation::SetopNone as i32,
        limit_option: protobuf::LimitOption::Default as i32,
        ..Default::default()
    };
    let result = ParseResult {
        version,
        stmts: vec![RawStmt {
            stmt: Some(Box::new(Node { node: Some(NodeEnum::SelectStmt(Box::new(select))) })),
            stmt_location: 0,
            stmt_len: 0,
        }],
    };
    pg_query::deparse(&result).ok()
}

// ---------------------------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------------------------

struct ResolveCtx<'a> {
    probes: &'a [Option<Outcome>],
    registry: &'a TypeRegistry,
    param_oids: &'a [u32],
}

impl ResolveCtx<'_> {
    /// One shape as (type, whether the value can never be null).
    fn node(&self, shape: &Shape, path: &str, unresolved: &mut Vec<String>) -> (Type, bool) {
        match shape {
            Shape::Object { fields, non_null } => {
                let fields = fields
                    .iter()
                    .map(|(key, value)| {
                        let (ty, non_null) = self.node(value, &join(path, key), unresolved);
                        Field { name: key.clone(), ty, nullable: !non_null }
                    })
                    .collect();
                (Type::Object { fields }, *non_null)
            }
            Shape::Array { items, non_null } => {
                let (of, items_non_null) = self.node(items, &join(path, "[]"), unresolved);
                (Type::List { of: Box::new(of), nullable_items: !items_non_null }, *non_null)
            }
            Shape::Column { probe, index } => match self.probes.get(*probe).and_then(|o| o.as_ref()).and_then(|o| o.columns.get(*index)) {
                Some(col) if col.type_oid != oid::UNKNOWN => (self.registry.render(col.type_oid), !col.nullable),
                _ => {
                    unresolved.push(display_path(path));
                    (Type::Unknown, false)
                }
            },
            Shape::Row { probe } => match self.probes.get(*probe).and_then(|o| o.as_ref()) {
                Some(outcome) => {
                    let fields = outcome
                        .columns
                        .iter()
                        .map(|col| {
                            let ty = if col.type_oid == oid::UNKNOWN {
                                unresolved.push(display_path(&join(path, &col.name)));
                                Type::Unknown
                            } else {
                                self.registry.render(col.type_oid)
                            };
                            Field { name: col.name.clone(), ty, nullable: col.nullable }
                        })
                        .collect();
                    (Type::Object { fields }, !outcome.outer_join)
                }
                None => {
                    unresolved.push(display_path(path));
                    (Type::Unknown, false)
                }
            },
            // A placeholder carries the type the statement's own describe gave it; whether it
            // arrived null is the caller's business, so the field stays nullable.
            Shape::Param(n) => match self.param_oids.get((*n as usize).saturating_sub(1)) {
                Some(o) => (self.registry.render(*o), false),
                None => {
                    unresolved.push(display_path(path));
                    (Type::Unknown, false)
                }
            },
            Shape::Unknown => {
                unresolved.push(display_path(path));
                (Type::Unknown, false)
            }
        }
    }
}

fn join(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.to_string()
    } else if key == "[]" {
        format!("{path}[]")
    } else {
        format!("{path}.{key}")
    }
}

fn display_path(path: &str) -> String {
    if path.is_empty() {
        "the whole value".to_string()
    } else {
        path.to_string()
    }
}

// ---------------------------------------------------------------------------------------------
// Parse-tree helpers
// ---------------------------------------------------------------------------------------------

pub(crate) fn as_select(node: &Node) -> Option<&SelectStmt> {
    match node.node.as_ref()? {
        NodeEnum::SelectStmt(s) => Some(s),
        _ => None,
    }
}

pub(crate) fn single_target(select: &SelectStmt) -> Option<&ResTarget> {
    if select.target_list.len() != 1 {
        return None;
    }
    match select.target_list[0].node.as_ref()? {
        NodeEnum::ResTarget(rt) => Some(rt),
        _ => None,
    }
}

/// The name the result column carries: its alias, else the name Postgres derives.
fn target_name(rt: &ResTarget) -> Option<String> {
    if !rt.name.is_empty() {
        return Some(rt.name.clone());
    }
    default_name(rt.val.as_deref()?)
}

fn default_name(node: &Node) -> Option<String> {
    match node.node.as_ref()? {
        NodeEnum::FuncCall(f) => func_name(f),
        NodeEnum::CoalesceExpr(_) => Some("coalesce".to_string()),
        NodeEnum::TypeCast(t) => t.type_name.as_ref().and_then(|tn| last_string(&tn.names)),
        NodeEnum::SubLink(s) => target_name(single_target(as_select(s.subselect.as_deref()?)?)?),
        _ => None,
    }
}

fn func_name(f: &protobuf::FuncCall) -> Option<String> {
    last_string(&f.funcname)
}

pub(crate) fn last_string(nodes: &[Node]) -> Option<String> {
    match nodes.last()?.node.as_ref()? {
        NodeEnum::String(s) => Some(s.sval.clone()),
        _ => None,
    }
}

fn string_node(value: &str) -> Node {
    Node { node: Some(NodeEnum::String(protobuf::String { sval: value.to_string() })) }
}

fn is_star(node: &Node) -> bool {
    match node.node.as_ref() {
        Some(NodeEnum::ColumnRef(c)) => {
            c.fields.last().is_some_and(|f| matches!(f.node.as_ref(), Some(NodeEnum::AStar(_))))
        }
        _ => false,
    }
}

/// A literal string, which is the only thing a key may be.
fn string_const(node: &Node) -> Option<String> {
    match node.node.as_ref()? {
        NodeEnum::AConst(c) if !c.isnull => match c.val.as_ref()? {
            protobuf::a_const::Val::Sval(s) => Some(s.sval.clone()),
            _ => None,
        },
        _ => None,
    }
}

/// A constant that is not NULL, which is what makes a `coalesce` non-null.
fn is_non_null_const(node: &Node) -> bool {
    match node.node.as_ref() {
        Some(NodeEnum::AConst(c)) => !c.isnull,
        Some(NodeEnum::TypeCast(t)) => t.arg.as_deref().is_some_and(is_non_null_const),
        Some(NodeEnum::AArrayExpr(_)) => true,
        _ => false,
    }
}

fn casts_to_json(t: &protobuf::TypeCast) -> bool {
    t.type_name
        .as_ref()
        .and_then(|tn| last_string(&tn.names))
        .is_some_and(|n| n == "json" || n == "jsonb")
}

/// A bare `u`, the alias form `to_jsonb` takes for a whole row.
fn bare_alias(node: &Node) -> Option<String> {
    match node.node.as_ref()? {
        NodeEnum::ColumnRef(c) if c.fields.len() == 1 => last_string(&c.fields),
        _ => None,
    }
}

/// The placeholder this expression is, through any casts.
fn param_number(node: &Node) -> Option<i32> {
    match node.node.as_ref()? {
        NodeEnum::ParamRef(p) => Some(p.number),
        NodeEnum::TypeCast(t) => param_number(t.arg.as_deref()?),
        _ => None,
    }
}

/// Whether a placeholder appears anywhere below this node.
fn contains_param<T: serde::Serialize>(node: &T) -> bool {
    fn hunt(v: &Value) -> bool {
        match v {
            Value::Object(map) => map.contains_key("ParamRef") || map.values().any(hunt),
            Value::Array(items) => items.iter().any(hunt),
            _ => false,
        }
    }
    serde_json::to_value(node).is_ok_and(|v| hunt(&v))
}

