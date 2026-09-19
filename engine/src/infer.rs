//! Expression columns, read off the SQL that computes them.
//!
//! The describe tier settles a plain column from its provenance: the table and attribute the
//! engine reports, the catalog's `NOT NULL`, the join analysis. An expression column has no
//! provenance. Postgres reports table oid 0 for `count(*)`, so there is no `attnotnull` to read,
//! and SQLite reports no declared type for it at all. The expression is in the statement,
//! though, and a small set of SQL constructs carries its own nullability and type:
//!
//! - `count(…)` is never NULL, over no rows and under `GROUP BY` alike. Every other aggregate
//!   is NULL over no rows and proves nothing.
//! - A literal other than `NULL` is never NULL.
//! - A cast, a `||`, a comparison, `AND`/`OR`/`NOT`, and (on Postgres) arithmetic are non-null
//!   when every operand is. SQLite turns a division by zero and a NaN into NULL, so it gets no
//!   arithmetic at all.
//! - `coalesce(a, b, …)` is non-null when any argument is; a `CASE` when every branch and an
//!   `ELSE` are; `EXISTS`, `IS [NOT] NULL`, `IS [NOT] TRUE` and `IS [NOT] DISTINCT FROM` never
//!   are NULL.
//! - A scalar subquery is non-null only when it is an ungrouped aggregate, which returns exactly
//!   one row, and its one column is non-null.
//! - A column reference inside an expression is a leaf. Its answer is the provenance one: the
//!   describer selects the leaves over the statement's own `FROM` as a probe and describes that,
//!   so the catalog and the outer-join degradation apply to it exactly as to a plain column.
//! - A set operation's arms are statements of their own. The describer describes each, so an arm
//!   that selects plain columns keeps their provenance, and a column is non-null when it is in
//!   every arm of a `UNION`, in either arm of an `INTERSECT`, and in the left arm of an `EXCEPT`.
//!
//! Everything else proves nothing and stays nullable, because a column typed non-null that can be
//! NULL crashes a client and a column left nullable only costs it a check. On Postgres a rule
//! also stands down when the database could resolve its operator, function, or cast to something
//! a user or an extension created ([`Trust`]).
//!
//! SQLite additionally gets a type for an expression column from the same rules: `count` is
//! `int8` as on Postgres, a literal is its kind, a cast is its target, `||` is text, a comparison
//! is a boolean, and `anti_html` is text. Postgres already describes every type, so there the
//! rules only ever settle nullability.
//!
//! SQLite has no parse tree of its own to offer, so both dialects are read with libpg_query, the
//! parser `jsonshape` already reads SQLite statements with. What it cannot parse, and what parses
//! as something the rules do not recognise, proves nothing.
//!
//! This module is pure: `plan` reads the parse tree, `resolve` and `combine` turn the probes'
//! describes into answers. The describer owns the round trips.

use crate::dialect::Dialect;
use crate::types::DescribedColumn;
use crate::jsonshape::{as_select, last_string, select_text, single_target, top_context};
use crate::pgtypes::oid;
use pg_query::protobuf::{self, node::Node as NodeEnum, Node, ParseResult, RawStmt, SelectStmt};
use std::collections::{HashMap, HashSet};

/// The operators whose Postgres built-ins return a value for every pair of non-null operands (or
/// raise: a division by zero is an error, never NULL). `!=` parses as `<>`.
pub const POSTGRES_OPERATORS: &[&str] = &["+", "-", "*", "/", "%", "||", "=", "<>", "<", ">", "<=", ">="];
/// SQLite's: concatenation and comparison only. Its arithmetic answers NULL for a division by
/// zero and for a NaN (`1e999 - 1e999`), so no arithmetic operator is total there.
const SQLITE_OPERATORS: &[&str] = &["||", "=", "==", "<>", "!=", "<", ">", "<=", ">="];
/// The functions a rule reads by name: `count`, plus the aggregates that make a subquery
/// return exactly one row.
pub const FUNCTIONS: &[&str] = &["count", "sum", "avg", "min", "max", "json_agg", "jsonb_agg"];

/// Postgres built-ins that answer NULL only for a NULL argument: a non-null input never yields
/// NULL (an invalid one raises an error instead). Nothing here answers NULL for a valid input
/// the way `nullif`, `substring` with no match or `array_length` of an empty array do, and no
/// aggregate is here: an aggregate over no rows is NULL.
pub const POSTGRES_TOTAL: &[&str] = &[
    "lower", "upper", "initcap", "length", "char_length", "character_length", "octet_length", "btrim", "ltrim", "rtrim", "md5",
    "reverse", "repeat", "replace", "left", "right", "lpad", "rpad", "translate", "ascii", "chr", "split_part", "strpos", "starts_with",
    "regexp_replace", "quote_ident", "quote_literal", "encode", "decode", "to_char", "to_hex", "to_timestamp", "to_date",
    "abs", "round", "floor", "ceil", "ceiling", "trunc", "sign", "sqrt", "cbrt", "exp", "ln", "log", "power", "mod",
    "date_trunc", "date_part", "extract", "age", "make_date", "make_timestamp", "make_timestamptz",
    "cardinality", "array_to_string", "array_append", "array_prepend", "array_cat", "array_remove",
    "to_json", "to_jsonb", "row_to_json", "json_typeof", "jsonb_typeof", "jsonb_strip_nulls", "jsonb_pretty",
];
/// Postgres built-ins that never answer NULL, whatever their arguments: the clock and random
/// functions, and the constructors that turn a NULL argument into an empty string or a JSON null.
pub const POSTGRES_NEVER_NULL: &[&str] = &[
    "now", "clock_timestamp", "statement_timestamp", "transaction_timestamp", "gen_random_uuid", "random",
    "concat", "concat_ws", "quote_nullable", "num_nulls", "num_nonnulls",
    "json_build_object", "json_build_array", "jsonb_build_object", "jsonb_build_array",
];
/// Postgres built-ins that answer NULL only when every argument is NULL.
pub const POSTGRES_ANY: &[&str] = &["greatest", "least"];
/// SQLite's cast targets with a type of their own, by the name libpg_query gives them
/// (`INTEGER` parses as `int4`, `REAL` as `float4`). A cast to anything else still carries its
/// argument's nullability on SQLite, but not a type: `CAST(x AS NUMERIC)` may be either class.
const SQLITE_CASTS: &[(&str, u32)] = &[
    ("int2", oid::INT2),
    ("int4", oid::INT4),
    ("int8", oid::INT8),
    ("float4", oid::FLOAT8),
    ("float8", oid::FLOAT8),
    ("text", oid::TEXT),
    ("varchar", oid::VARCHAR),
];
/// SQLite cast targets whose result is never NULL for a non-null argument, shown for `INTEGER`,
/// `REAL` and `TEXT`: `CAST('abc' AS INTEGER)` is 0, not NULL.
const SQLITE_TOTAL_CASTS: &[&str] = &["int2", "int4", "int8", "float4", "float8", "text", "varchar"];

/// What the SQL says about one expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Never NULL, of this type where the SQL names one.
    NonNull(Option<u32>),
    /// Nothing the rules recognise: nullable and untyped.
    Unknown,
    /// The inner expression's type, nullable whatever the inner proves: a `CASE` with no `ELSE`,
    /// a scalar subquery that may match no row, SQLite's `total()`.
    Nullable(Box<Expr>),
    /// Column `n` of the leaf probe: a column reference, answered by its provenance.
    Leaf(usize),
    /// Non-null when every operand is: an operator, a cast, a `CASE` with an `ELSE`.
    Every(Vec<Expr>, Ty),
    /// Non-null when any operand is: `coalesce`.
    Any(Vec<Expr>, Ty),
}

/// The type an [`Expr::Every`] or [`Expr::Any`] carries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ty {
    /// This type, whatever the operands are: `||` is text, a comparison a boolean.
    Is(u32),
    /// The operands' type when they all agree on one: `coalesce`, `CASE`.
    Shared,
    /// Nothing the build can name.
    Unknown,
}

/// The two arms of a set operation at the top of a statement, as statements of their own.
#[derive(Debug, Clone, PartialEq)]
pub struct SetOp {
    pub kind: SetKind,
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetKind {
    Union,
    Intersect,
    Except,
}

/// What one statement's expression columns need.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    /// The result columns before the first `*` of the target list, in order: `None` where the
    /// build claims nothing, which is every plain column reference, since its provenance already
    /// answers.
    pub head: Vec<Option<Expr>>,
    /// The result columns after the last `*`, in order. Empty unless the target list has one.
    pub tail: Vec<Option<Expr>>,
    /// Whether a `*` sits between the two, so the result has columns the plan cannot count.
    pub star: bool,
    /// `SELECT <the leaves> FROM <the statement's FROM>`, when an expression reads a column.
    pub leaves: Option<String>,
    /// A set operation at the top of the statement; nothing else in the plan is set then.
    pub set_op: Option<SetOp>,
}

/// One column's answer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fact {
    pub non_null: bool,
    pub type_oid: Option<u32>,
}

/// The names a rule reads that the database could resolve to something the build has not read.
/// On Postgres that is any operator, function, cast or type a user or an extension created: it
/// can shadow a built-in for its own argument types and answer NULL where the built-in never
/// does. A rule whose name is shadowed proves nothing.
#[derive(Debug, Clone, Default)]
pub struct Trust {
    /// Operator and function names with a definition outside the built-ins.
    pub shadowed: HashSet<String>,
    /// The built-in type names a cast may target and keep its argument's nullability. `None`
    /// trusts every name, which is what SQLite gets: nothing there can define a cast.
    pub cast_targets: Option<HashSet<String>>,
    /// Functions the host registers on its own connection, by name, with the type oid each
    /// returns: NULL only when an argument is NULL. Anti's `anti_html` on SQLite is one.
    pub total: HashMap<String, u32>,
    /// Whether a parameter is a non-null value. A host whose callers cannot pass null (Lando
    /// types a parameter without it) sets this; by default a placeholder proves nothing.
    pub non_null_params: bool,
    /// The parameter numbers that may be null all the same.
    pub nullable_params: HashSet<i32>,
}

impl Trust {
    fn builtin(&self, name: &str) -> bool {
        !self.shadowed.contains(name)
    }

    fn cast(&self, name: &str) -> bool {
        self.cast_targets.as_ref().is_none_or(|t| t.contains(name))
    }
}

/// Read one rewritten statement's expression columns. SQLite's `?n` placeholders are read as
/// `$n`, which is how libpg_query spells one; nothing runs against the database here.
pub fn plan(sql: &str, dialect: Dialect, trust: &Trust) -> Plan {
    let text = match dialect {
        Dialect::Postgres => sql.to_string(),
        Dialect::Sqlite | Dialect::Mysql => dollar_placeholders(sql),
    };
    let Ok(parsed) = pg_query::parse(&text) else { return Plan::default() };
    let version = parsed.protobuf.version;
    let Some(top) = parsed.protobuf.stmts.first().and_then(|s| s.stmt.as_deref()).and_then(|n| n.node.as_ref()) else {
        return Plan::default();
    };
    if let NodeEnum::SelectStmt(s) = top {
        if let Some(kind) = set_kind(s) {
            let arm = |a: &Option<Box<SelectStmt>>| a.as_deref().and_then(|a| arm_text(version, a, s.with_clause.as_ref()));
            return match (arm(&s.larg), arm(&s.rarg)) {
                (Some(left), Some(right)) => Plan { set_op: Some(SetOp { kind, left, right }), ..Plan::default() },
                _ => Plan::default(),
            };
        }
        if !s.values_lists.is_empty() {
            return values_plan(s, dialect, trust);
        }
    }
    let Some((targets, from, with)) = top_context(top) else { return Plan::default() };
    // A grouping set nulls a grouping column in its super-aggregate rows, which a probe over the
    // `FROM` alone would never show: a leaf there proves nothing.
    let grouped = matches!(top, NodeEnum::SelectStmt(s) if s.group_clause.iter().any(|g| matches!(g.node, Some(NodeEnum::GroupingSet(_)))));
    let mut reader = Reader { dialect, trust, leaves: (!grouped).then(Vec::new) };
    let mut plan = Plan::default();
    for target in &targets {
        let Some(NodeEnum::ResTarget(rt)) = target.node.as_ref() else { continue };
        let Some(val) = rt.val.as_deref() else { continue };
        if is_star(val) {
            plan.star = true;
            plan.tail.clear();
            continue;
        }
        // A plain column reference is the describe tier's to answer.
        let claim = if matches!(val.node, Some(NodeEnum::ColumnRef(_))) { None } else { Some(reader.read(val)) };
        if plan.star {
            plan.tail.push(claim);
        } else {
            plan.head.push(claim);
        }
    }
    let leaves = reader.leaves.unwrap_or_default();
    if !leaves.is_empty() {
        plan.leaves = select_text(version, &leaves, &from, with.as_ref());
    }
    plan
}

impl Plan {
    /// The claim for each of `count` result columns, in order, or nothing when the target list
    /// and the describe disagree about how many there are.
    pub fn claims(&self, count: usize) -> Vec<Option<&Expr>> {
        let known = self.head.len() + self.tail.len();
        if (!self.star && known != count) || known > count {
            return vec![None; count];
        }
        let mut out: Vec<Option<&Expr>> = self.head.iter().map(Option::as_ref).collect();
        out.resize(count - self.tail.len(), None);
        out.extend(self.tail.iter().map(Option::as_ref));
        out
    }

    /// Whether any claim reads a leaf, so the leaf probe is worth its describe.
    pub fn reads_leaves(&self) -> bool {
        self.head.iter().chain(&self.tail).flatten().any(reads_leaf)
    }
}

/// One expression's answer, given the leaf probe's described columns.
pub fn resolve(expr: &Expr, leaves: Option<&[DescribedColumn]>) -> Fact {
    const NOTHING: Fact = Fact { non_null: false, type_oid: None };
    match expr {
        Expr::NonNull(type_oid) => Fact { non_null: true, type_oid: *type_oid },
        Expr::Unknown => NOTHING,
        Expr::Nullable(inner) => Fact { non_null: false, ..resolve(inner, leaves) },
        Expr::Leaf(i) => match leaves.and_then(|l| l.get(*i)) {
            Some(col) => Fact { non_null: !col.nullable, type_oid: (col.type_oid != oid::UNKNOWN).then_some(col.type_oid) },
            None => NOTHING,
        },
        Expr::Every(operands, ty) => {
            let facts: Vec<Fact> = operands.iter().map(|o| resolve(o, leaves)).collect();
            Fact { non_null: !facts.is_empty() && facts.iter().all(|f| f.non_null), type_oid: ty.of(&facts) }
        }
        Expr::Any(operands, ty) => {
            let facts: Vec<Fact> = operands.iter().map(|o| resolve(o, leaves)).collect();
            Fact { non_null: facts.iter().any(|f| f.non_null), type_oid: ty.of(&facts) }
        }
    }
}

/// A set operation's columns from its arms' describes: non-null in every arm of a `UNION`, in
/// either arm of an `INTERSECT` (a row it returns is not distinct from a row of each arm, and
/// NULL is not distinct only from NULL), in the left arm of an `EXCEPT` (every row it returns is
/// one of the left arm's). A type is claimed only where both arms describe the same one.
pub fn combine(kind: SetKind, left: &[DescribedColumn], right: &[DescribedColumn]) -> Option<Vec<Fact>> {
    if left.len() != right.len() {
        return None;
    }
    Some(
        left.iter()
            .zip(right)
            .map(|(l, r)| Fact {
                non_null: match kind {
                    SetKind::Union => !l.nullable && !r.nullable,
                    SetKind::Intersect => !l.nullable || !r.nullable,
                    SetKind::Except => !l.nullable,
                },
                type_oid: (l.type_oid == r.type_oid && l.type_oid != oid::UNKNOWN).then_some(l.type_oid),
            })
            .collect(),
    )
}

/// Whether a subquery returns exactly one row once it aggregates: no grouping, no `HAVING` to
/// drop the one group, no `LIMIT` or `OFFSET` to skip it, and no set operation or `VALUES` to be
/// something other than one `SELECT`. Postgres: "If a query contains aggregate function calls,
/// but no GROUP BY clause, grouping still occurs: the result is a single group row". Whether it
/// does aggregate is the caller's to show.
pub fn single_row(select: &SelectStmt) -> bool {
    set_kind(select).is_none()
        && select.values_lists.is_empty()
        && select.group_clause.is_empty()
        && select.having_clause.is_none()
        && select.limit_count.is_none()
        && select.limit_offset.is_none()
}

/// Whether an aggregate call sits anywhere in this target expression outside a subquery, which makes
/// the `SELECT` holding it an aggregate query: the reading `single_row` leans on, for a caller that
/// proves a row count (`cardinality`) rather than a nullability.
pub fn aggregates(expr: &Node, dialect: Dialect, trust: &Trust) -> bool {
    Reader { dialect, trust, leaves: None }.aggregates(expr)
}

impl Ty {
    fn of(self, facts: &[Fact]) -> Option<u32> {
        match self {
            Ty::Is(o) => Some(o),
            Ty::Unknown => None,
            Ty::Shared => {
                let first = facts.first()?.type_oid?;
                facts.iter().all(|f| f.type_oid == Some(first)).then_some(first)
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Recognition
// ---------------------------------------------------------------------------------------------

struct Reader<'a> {
    dialect: Dialect,
    trust: &'a Trust,
    /// The column references read so far, in leaf order; `None` where a reference proves nothing.
    leaves: Option<Vec<Node>>,
}

impl Reader<'_> {
    fn read(&mut self, expr: &Node) -> Expr {
        let bool_ty = Ty::Is(oid::BOOL);
        match expr.node.as_ref() {
            Some(NodeEnum::AConst(c)) => {
                if c.isnull {
                    return Expr::Unknown;
                }
                Expr::NonNull(match c.val.as_ref() {
                    Some(protobuf::a_const::Val::Ival(_)) => Some(oid::INT4),
                    // A literal past int4, or one with a fraction or an exponent.
                    Some(protobuf::a_const::Val::Fval(f)) => Some(if f.fval.parse::<i64>().is_ok() { oid::INT8 } else { oid::FLOAT8 }),
                    Some(protobuf::a_const::Val::Boolval(_)) => Some(oid::BOOL),
                    Some(protobuf::a_const::Val::Sval(_)) => Some(oid::TEXT),
                    _ => None,
                })
            }
            Some(NodeEnum::ColumnRef(_)) => match self.leaves.as_mut() {
                Some(leaves) if !is_star(expr) => {
                    leaves.push(expr.clone());
                    Expr::Leaf(leaves.len() - 1)
                }
                _ => Expr::Unknown,
            },
            // `ROW(a, b)` and `(a, b)` build a row whatever their fields hold, and a cast of one
            // to a composite type only relabels it: no cast can be defined from `record`.
            Some(NodeEnum::RowExpr(_)) if self.dialect == Dialect::Postgres => Expr::NonNull(None),
            Some(NodeEnum::TypeCast(t)) if self.dialect == Dialect::Postgres && matches!(t.arg.as_deref().and_then(|a| a.node.as_ref()), Some(NodeEnum::RowExpr(_))) => Expr::NonNull(None),
            Some(NodeEnum::TypeCast(t)) => {
                let (Some(arg), Some(target)) = (t.arg.as_deref(), t.type_name.as_ref()) else { return Expr::Unknown };
                if !target.array_bounds.is_empty() || target.setof || target.pct_type {
                    return Expr::Unknown;
                }
                let Some(name) = builtin_name(&target.names) else { return Expr::Unknown };
                let operand = self.read(arg);
                match self.dialect {
                    Dialect::Postgres if self.trust.cast(&name) => Expr::Every(vec![operand], Ty::Unknown),
                    Dialect::Sqlite | Dialect::Mysql if SQLITE_TOTAL_CASTS.contains(&name.as_str()) => {
                        let ty = SQLITE_CASTS.iter().find(|(n, _)| *n == name).map_or(Ty::Unknown, |(_, o)| Ty::Is(*o));
                        Expr::Every(vec![operand], ty)
                    }
                    _ => Expr::Unknown,
                }
            }
            Some(NodeEnum::AExpr(a)) => {
                let distinct = [protobuf::AExprKind::AexprDistinct as i32, protobuf::AExprKind::AexprNotDistinct as i32];
                if distinct.contains(&a.kind) {
                    return Expr::NonNull(Some(oid::BOOL));
                }
                if a.kind != protobuf::AExprKind::AexprOp as i32 {
                    return Expr::Unknown;
                }
                let Some(op) = builtin_name(&a.name) else { return Expr::Unknown };
                let accepted = match self.dialect {
                    Dialect::Postgres => POSTGRES_OPERATORS.contains(&op.as_str()) && self.trust.builtin(&op),
                    Dialect::Sqlite | Dialect::Mysql => SQLITE_OPERATORS.contains(&op.as_str()),
                };
                let Some(right) = a.rexpr.as_deref() else { return Expr::Unknown };
                // A prefix operator: only a sign is one of the accepted set.
                let left = match a.lexpr.as_deref() {
                    Some(l) => Some(l),
                    None if op == "-" || op == "+" => None,
                    None => return Expr::Unknown,
                };
                if !accepted {
                    return Expr::Unknown;
                }
                let ty = match op.as_str() {
                    "||" if self.dialect != Dialect::Postgres => Ty::Is(oid::TEXT),
                    "=" | "==" | "<>" | "!=" | "<" | ">" | "<=" | ">=" => bool_ty,
                    _ => Ty::Unknown,
                };
                let mut operands: Vec<Expr> = left.into_iter().map(|l| self.read(l)).collect();
                operands.push(self.read(right));
                Expr::Every(operands, ty)
            }
            Some(NodeEnum::BoolExpr(b)) if !b.args.is_empty() => Expr::Every(b.args.iter().map(|a| self.read(a)).collect(), bool_ty),
            Some(NodeEnum::NullTest(_)) | Some(NodeEnum::BooleanTest(_)) => Expr::NonNull(Some(oid::BOOL)),
            // `ARRAY[...]` always builds an array, whatever its elements hold; the describe already
            // names the element type.
            Some(NodeEnum::AArrayExpr(_)) => Expr::NonNull(None),
            Some(NodeEnum::CoalesceExpr(c)) if !c.args.is_empty() => Expr::Any(c.args.iter().map(|a| self.read(a)).collect(), Ty::Shared),
            Some(NodeEnum::CaseExpr(c)) => {
                let mut results = Vec::with_capacity(c.args.len() + 1);
                for when in &c.args {
                    let Some(NodeEnum::CaseWhen(w)) = when.node.as_ref() else { return Expr::Unknown };
                    let Some(result) = w.result.as_deref() else { return Expr::Unknown };
                    results.push(self.read(result));
                }
                match c.defresult.as_deref() {
                    Some(otherwise) => {
                        results.push(self.read(otherwise));
                        Expr::Every(results, Ty::Shared)
                    }
                    // No `ELSE` is `ELSE NULL`.
                    None => Expr::Nullable(Box::new(Expr::Every(results, Ty::Shared))),
                }
            }
            Some(NodeEnum::FuncCall(f)) => self.call(f),
            Some(NodeEnum::SubLink(s)) => self.sublink(s),
            // `current_timestamp`, `current_user` and the rest of the SQL value functions.
            Some(NodeEnum::SqlvalueFunction(_)) => Expr::NonNull(None),
            // Where the host types its parameters without null, a parameter is a non-null value.
            Some(NodeEnum::ParamRef(p)) if self.trust.non_null_params && !self.trust.nullable_params.contains(&p.number) => Expr::NonNull(None),
            _ => Expr::Unknown,
        }
    }

    fn call(&mut self, f: &protobuf::FuncCall) -> Expr {
        let Some(name) = builtin_name(&f.funcname) else { return Expr::Unknown };
        if f.agg_within_group || f.func_variadic {
            return Expr::Unknown;
        }
        match (self.dialect, name.as_str(), f.args.len()) {
            // `count(*)`, `count(x)`, `count(DISTINCT x)`, filtered or windowed: a count of
            // nothing is 0.
            (_, "count", 0 | 1) if self.trust.builtin("count") => Expr::NonNull(Some(oid::INT8)),
            // "total() returns 0.0" over no rows, but a sum of `1e999` and `-1e999` is NaN, which
            // SQLite stores as NULL: a float, and nullable.
            (Dialect::Sqlite, "total", 1) => Expr::Nullable(Box::new(Expr::NonNull(Some(oid::FLOAT8)))),
            // "The ifnull() function is equivalent to coalesce() with two arguments."
            (Dialect::Sqlite, "ifnull", 2) => Expr::Any(f.args.iter().map(|a| self.read(a)).collect(), Ty::Shared),
            (Dialect::Postgres, n, _) if POSTGRES_NEVER_NULL.contains(&n) && self.trust.builtin(n) => Expr::NonNull(None),
            (Dialect::Postgres, n, _) if POSTGRES_ANY.contains(&n) && self.trust.builtin(n) => Expr::Any(f.args.iter().map(|a| self.read(a)).collect(), Ty::Shared),
            (Dialect::Postgres, n, _) if POSTGRES_TOTAL.contains(&n) && self.trust.builtin(n) => Expr::Every(f.args.iter().map(|a| self.read(a)).collect(), Ty::Unknown),
            _ => match self.trust.total.get(name.as_str()) {
                Some(&ty) => Expr::Every(f.args.iter().map(|a| self.read(a)).collect(), Ty::Is(ty)),
                None => Expr::Unknown,
            },
        }
    }

    fn sublink(&mut self, s: &protobuf::SubLink) -> Expr {
        if s.sub_link_type == protobuf::SubLinkType::ExistsSublink as i32 {
            return Expr::NonNull(Some(oid::BOOL));
        }
        // `ARRAY(SELECT …)` is an array whatever the subquery returns: empty for no rows.
        if s.sub_link_type == protobuf::SubLinkType::ArraySublink as i32 {
            return Expr::NonNull(None);
        }
        if s.sub_link_type != protobuf::SubLinkType::ExprSublink as i32 {
            return Expr::Unknown;
        }
        let Some(select) = s.subselect.as_deref().and_then(as_select) else { return Expr::Unknown };
        let Some(inner) = single_target(select).and_then(|rt| rt.val.as_deref()) else { return Expr::Unknown };
        // The leaf probe selects over the outer `FROM`, which a reference inside a subquery does
        // not read, so the inner expression is read without leaves.
        let mut inner_reader = Reader { dialect: self.dialect, trust: self.trust, leaves: None };
        let value = inner_reader.read(inner);
        if single_row(select) && self.aggregates(inner) {
            value
        } else {
            Expr::Nullable(Box::new(value))
        }
    }

    /// Whether an aggregate call sits anywhere in this target expression outside a subquery,
    /// which makes the `SELECT` holding it an aggregate query. Only a name that is an aggregate
    /// at this arity counts: SQLite's two-argument `max(a, b)` is a scalar function.
    fn aggregates(&self, expr: &Node) -> bool {
        let children: Vec<&Node> = match expr.node.as_ref() {
            Some(NodeEnum::FuncCall(f)) => {
                let aggregate = f.over.is_none()
                    && builtin_name(&f.funcname).is_some_and(|name| {
                        let arity = match name.as_str() {
                            "count" => f.args.len() <= 1,
                            "sum" | "avg" | "min" | "max" | "json_agg" | "jsonb_agg" => f.args.len() == 1,
                            "total" | "json_group_array" => self.dialect == Dialect::Sqlite && f.args.len() == 1,
                            _ => false,
                        };
                        arity && self.trust.builtin(&name)
                    });
                if aggregate {
                    return true;
                }
                f.args.iter().collect()
            }
            Some(NodeEnum::AExpr(a)) => a.lexpr.iter().chain(a.rexpr.iter()).map(|b| b.as_ref()).collect(),
            Some(NodeEnum::TypeCast(t)) => t.arg.iter().map(|b| b.as_ref()).collect(),
            Some(NodeEnum::CoalesceExpr(c)) => c.args.iter().collect(),
            Some(NodeEnum::BoolExpr(b)) => b.args.iter().collect(),
            Some(NodeEnum::CaseExpr(c)) => {
                let mut out: Vec<&Node> = c.defresult.iter().map(|b| b.as_ref()).collect();
                for when in &c.args {
                    if let Some(NodeEnum::CaseWhen(w)) = when.node.as_ref() {
                        out.extend(w.expr.iter().chain(w.result.iter()).map(|b| b.as_ref()));
                    }
                }
                out
            }
            _ => Vec::new(),
        };
        children.into_iter().any(|c| self.aggregates(c))
    }
}

/// A bare `VALUES (…), (…)` arm: a column is non-null when it is in every row.
fn values_plan(s: &SelectStmt, dialect: Dialect, trust: &Trust) -> Plan {
    let mut reader = Reader { dialect, trust, leaves: None };
    let mut rows: Vec<Vec<Expr>> = Vec::new();
    for row in &s.values_lists {
        let Some(NodeEnum::List(list)) = row.node.as_ref() else { return Plan::default() };
        rows.push(list.items.iter().map(|v| reader.read(v)).collect());
    }
    let width = rows.first().map_or(0, Vec::len);
    if rows.iter().any(|r| r.len() != width) {
        return Plan::default();
    }
    let head = (0..width).map(|i| Some(Expr::Every(rows.iter().map(|r| r[i].clone()).collect(), Ty::Shared))).collect();
    Plan { head, ..Plan::default() }
}

fn set_kind(s: &SelectStmt) -> Option<SetKind> {
    match s.op {
        op if op == protobuf::SetOperation::SetopUnion as i32 => Some(SetKind::Union),
        op if op == protobuf::SetOperation::SetopIntersect as i32 => Some(SetKind::Intersect),
        op if op == protobuf::SetOperation::SetopExcept as i32 => Some(SetKind::Except),
        _ => None,
    }
}

/// One arm of a set operation as a statement of its own, carrying the statement's `WITH` so an
/// arm reading a CTE still resolves.
fn arm_text(version: i32, arm: &SelectStmt, with: Option<&protobuf::WithClause>) -> Option<String> {
    let mut arm = arm.clone();
    if arm.with_clause.is_none() {
        arm.with_clause = with.cloned();
    }
    let result = ParseResult {
        version,
        stmts: vec![RawStmt {
            stmt: Some(Box::new(Node { node: Some(NodeEnum::SelectStmt(Box::new(arm))) })),
            stmt_location: 0,
            stmt_len: 0,
        }],
    };
    pg_query::deparse(&result).ok()
}

/// The unqualified name of an operator, function, or type the rules read, or `None` when it is
/// schema-qualified with anything but Postgres's own catalog: a qualified name is somebody else's.
/// `CAST(x AS INTEGER)` parses as `pg_catalog.int4` on SQLite too, which is the parser's
/// spelling of a keyword, not a schema SQLite has.
fn builtin_name(names: &[Node]) -> Option<String> {
    let name = last_string(names)?;
    match names.len() {
        1 => Some(name),
        2 if last_string(&names[..1]).as_deref() == Some("pg_catalog") => Some(name),
        _ => None,
    }
}

fn is_star(node: &Node) -> bool {
    match node.node.as_ref() {
        Some(NodeEnum::ColumnRef(c)) => c.fields.last().is_some_and(|f| matches!(f.node.as_ref(), Some(NodeEnum::AStar(_)))),
        _ => false,
    }
}

fn reads_leaf(expr: &Expr) -> bool {
    match expr {
        Expr::Leaf(_) => true,
        Expr::Nullable(inner) => reads_leaf(inner),
        Expr::Every(operands, _) | Expr::Any(operands, _) => operands.iter().any(reads_leaf),
        Expr::NonNull(_) | Expr::Unknown => false,
    }
}

/// SQLite's `?n` as `$n`, outside strings, quoted identifiers, and comments. The build's rewrite
/// only ever writes the numbered form.
pub fn dollar_placeholders(sql: &str) -> String {
    let b = sql.as_bytes();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0;
    let mut copied = 0;
    while i < b.len() {
        match b[i] {
            b'\'' | b'"' | b'`' | b'[' => {
                let close = if b[i] == b'[' { b']' } else { b[i] };
                i += 1;
                while i < b.len() && b[i] != close {
                    i += 1;
                }
                i += 1;
            }
            b'-' if b.get(i + 1) == Some(&b'-') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                i += 2;
                while i < b.len() && !(b[i] == b'*' && b.get(i + 1) == Some(&b'/')) {
                    i += 1;
                }
                i += 2;
            }
            b'?' if b.get(i + 1).is_some_and(u8::is_ascii_digit) => {
                out.push_str(&sql[copied..i]);
                out.push('$');
                i += 1;
                copied = i;
            }
            _ => i += 1,
        }
    }
    out.push_str(&sql[copied.min(sql.len())..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pg(sql: &str) -> Plan {
        plan(sql, Dialect::Postgres, &Trust::default())
    }

    fn lite(sql: &str) -> Plan {
        plan(sql, Dialect::Sqlite, &Trust::default())
    }

    /// The first column's answer with no leaf probe.
    fn first(p: &Plan) -> Fact {
        resolve(p.head[0].as_ref().expect("a claim"), None)
    }

    fn col(nullable: bool, type_oid: u32) -> DescribedColumn {
        DescribedColumn { name: "a".into(), type_oid, type_name: String::new(), nullable }
    }

    #[test]
    fn a_count_is_never_null_and_other_aggregates_are() {
        assert_eq!(first(&pg("SELECT count(*) AS n FROM todos")), Fact { non_null: true, type_oid: Some(oid::INT8) });
        assert!(first(&pg("SELECT count(t.id) FROM todos t GROUP BY t.owner_id")).non_null);
        assert!(first(&pg("SELECT count(DISTINCT t.id) FILTER (WHERE t.done) FROM todos t")).non_null);
        assert!(first(&pg("SELECT count(*) OVER (ORDER BY id ROWS BETWEEN 5 FOLLOWING AND 6 FOLLOWING) FROM todos")).non_null);
        // Every other aggregate is NULL over no rows.
        for sql in ["SELECT sum(n) FROM t", "SELECT max(n) FROM t", "SELECT min(n) FROM t", "SELECT avg(n) FROM t", "SELECT json_agg(n) FROM t"] {
            assert!(!first(&pg(sql)).non_null, "{sql}");
        }
        // A count cast keeps the count's answer.
        assert!(first(&pg("SELECT CAST(count(*) AS integer) AS total FROM todos")).non_null);
        assert!(first(&pg("SELECT count(*)::int FROM todos")).non_null);
    }

    #[test]
    fn literals_are_non_null_and_null_is_not() {
        assert!(first(&pg("SELECT 'ok'")).non_null);
        assert!(first(&pg("SELECT 1")).non_null);
        assert!(first(&pg("SELECT true")).non_null);
        assert!(!first(&pg("SELECT NULL")).non_null);
        assert!(!first(&pg("SELECT NULL::int")).non_null);
        assert_eq!(first(&lite("SELECT 'ok'")).type_oid, Some(oid::TEXT));
        assert_eq!(first(&lite("SELECT 1")).type_oid, Some(oid::INT4));
        assert_eq!(first(&lite("SELECT 9999999999")).type_oid, Some(oid::INT8));
        assert_eq!(first(&lite("SELECT 1.5")).type_oid, Some(oid::FLOAT8));
        assert_eq!(first(&lite("SELECT true")).type_oid, Some(oid::BOOL));
    }

    #[test]
    fn coalesce_needs_one_non_null_argument_and_case_needs_every_branch() {
        assert!(first(&pg("SELECT coalesce(NULL, 0)")).non_null);
        assert!(!first(&pg("SELECT coalesce(NULL, NULL::int)")).non_null);
        assert!(first(&pg("SELECT CASE WHEN true THEN 1 ELSE 2 END")).non_null);
        assert!(!first(&pg("SELECT CASE WHEN true THEN 1 ELSE NULL END")).non_null);
        // No `ELSE` is `ELSE NULL`.
        assert!(!first(&pg("SELECT CASE WHEN true THEN 1 END")).non_null);
        assert!(!first(&pg("SELECT CASE 1 WHEN 1 THEN 'a' END")).non_null);
    }

    #[test]
    fn tests_and_exists_are_never_null_and_nullif_is_not_an_operator() {
        assert!(first(&pg("SELECT EXISTS (SELECT 1 FROM todos)")).non_null);
        assert!(first(&pg("SELECT NULL IS NULL")).non_null);
        assert!(first(&pg("SELECT NULL::bool IS NOT TRUE")).non_null);
        assert!(first(&pg("SELECT NULL IS DISTINCT FROM 1")).non_null);
        assert!(!first(&pg("SELECT nullif(1, 1)")).non_null);
        assert!(!first(&pg("SELECT 1 IN (1, NULL)")).non_null);
        assert!(!first(&pg("SELECT 'a' LIKE 'b'")).non_null, "LIKE is outside the accepted set");
    }

    #[test]
    fn operators_are_non_null_on_non_null_operands() {
        assert!(first(&pg("SELECT 1 + 2 * 3")).non_null);
        assert!(first(&pg("SELECT 'a' || 'b'")).non_null);
        assert!(first(&pg("SELECT 1 <> 2 AND NOT false")).non_null);
        assert!(first(&pg("SELECT -1")).non_null);
        assert!(!first(&pg("SELECT 1 + NULL")).non_null);
        // Only built-in names: a qualified operator is somebody else's.
        assert!(!first(&pg("SELECT 1 OPERATOR(myschema.+) 2")).non_null);
        assert!(first(&pg("SELECT 1 OPERATOR(pg_catalog.+) 2")).non_null);
        // `!=` parses as `<>`.
        assert!(first(&pg("SELECT 1 != 2")).non_null);
    }

    #[test]
    fn sqlite_has_no_total_arithmetic() {
        // `1/0` and `1e999 - 1e999` are both NULL on SQLite.
        assert!(!first(&lite("SELECT 1 / 0")).non_null);
        assert!(!first(&lite("SELECT 1 + 2")).non_null);
        assert_eq!(first(&lite("SELECT 'a' || 1")), Fact { non_null: true, type_oid: Some(oid::TEXT) });
        assert_eq!(first(&lite("SELECT 1 < 2")), Fact { non_null: true, type_oid: Some(oid::BOOL) });
        // `total()` is a float that a NaN makes NULL; `sum` has no type, since a column declared
        // INTEGER may still hold a real.
        assert_eq!(first(&lite("SELECT total(n) FROM t")), Fact { non_null: false, type_oid: Some(oid::FLOAT8) });
        assert_eq!(first(&lite("SELECT sum(n) FROM t")), Fact { non_null: false, type_oid: None });
        assert_eq!(first(&lite("SELECT count(*) AS total FROM todos WHERE owner_id = ?1")), Fact { non_null: true, type_oid: Some(oid::INT8) });
        assert_eq!(first(&lite("SELECT ifnull(NULL, 'x')")), Fact { non_null: true, type_oid: None });
    }

    #[test]
    fn a_cast_carries_its_argument_and_on_sqlite_its_target() {
        assert_eq!(first(&lite("SELECT CAST(count(*) AS INTEGER) AS n FROM t")), Fact { non_null: true, type_oid: Some(oid::INT4) });
        assert_eq!(first(&lite("SELECT CAST(1 AS REAL)")), Fact { non_null: true, type_oid: Some(oid::FLOAT8) });
        assert_eq!(first(&lite("SELECT CAST(1 AS TEXT)")), Fact { non_null: true, type_oid: Some(oid::TEXT) });
        assert_eq!(first(&lite("SELECT CAST(1 AS BIGINT)")), Fact { non_null: true, type_oid: Some(oid::INT8) });
        // A class SQLite may answer either way is neither typed nor proven.
        assert_eq!(first(&lite("SELECT CAST(1 AS NUMERIC)")), Fact { non_null: false, type_oid: None });
        assert!(!first(&lite("SELECT CAST(NULL AS INTEGER)")).non_null);
        // On Postgres a cast needs a target the catalog says only the built-ins reach.
        let shadowed = Trust { shadowed: HashSet::new(), cast_targets: Some(["text".to_string()].into_iter().collect()), ..Default::default() };
        assert!(first(&plan("SELECT 1::text", Dialect::Postgres, &shadowed)).non_null);
        assert!(!first(&plan("SELECT 1::int4", Dialect::Postgres, &shadowed)).non_null);
        assert!(!first(&pg("SELECT '{}'::uuid[]")).non_null, "an array cast is outside the set");
    }

    #[test]
    fn a_shadowed_name_proves_nothing() {
        let trust = Trust { shadowed: ["count".to_string(), "=".to_string()].into_iter().collect(), cast_targets: None, ..Default::default() };
        assert!(!first(&plan("SELECT count(*) FROM t", Dialect::Postgres, &trust)).non_null);
        assert!(!first(&plan("SELECT 1 = 1", Dialect::Postgres, &trust)).non_null);
        assert!(first(&plan("SELECT 1 + 1", Dialect::Postgres, &trust)).non_null);
    }

    #[test]
    fn a_scalar_subquery_is_non_null_only_as_an_ungrouped_aggregate() {
        assert!(first(&pg("SELECT (SELECT count(*) FROM todos t WHERE t.owner_id = o.id) AS n FROM owners o")).non_null);
        assert!(first(&pg("SELECT (SELECT coalesce(max(t.n), 0) FROM todos t) AS n")).non_null);
        // Not an aggregate: it may match no row.
        assert!(!first(&pg("SELECT (SELECT 1 FROM todos LIMIT 1) AS n")).non_null);
        // An aggregate whose one row a clause can take away.
        for sql in [
            "SELECT (SELECT count(*) FROM todos GROUP BY owner_id) AS n",
            "SELECT (SELECT count(*) FROM todos HAVING false) AS n",
            "SELECT (SELECT count(*) FROM todos LIMIT 0) AS n",
            "SELECT (SELECT count(*) FROM todos OFFSET 1) AS n",
        ] {
            assert!(!first(&pg(sql)).non_null, "{sql}");
        }
        // A window call is not an aggregate, and neither is SQLite's two-argument `max`.
        assert!(!first(&pg("SELECT (SELECT coalesce(count(*) OVER (), 0) FROM todos) AS n")).non_null);
        assert!(!first(&lite("SELECT (SELECT coalesce(max(a, b), 0) FROM t) AS n")).non_null);
        // A non-null aggregate still needs a non-null value.
        assert!(!first(&pg("SELECT (SELECT max(n) FROM todos) AS n")).non_null);
    }

    #[test]
    fn a_column_reference_in_an_expression_is_a_leaf_over_the_same_from() {
        let p = pg("SELECT t.title || ' by ' || u.name AS label, t.id FROM todos t LEFT JOIN users u ON u.id = t.owner_id");
        assert_eq!(p.leaves.as_deref(), Some("SELECT t.title AS a0, u.name AS a1 FROM todos t LEFT JOIN users u ON u.id = t.owner_id"));
        assert!(p.reads_leaves());
        assert_eq!(p.head[1], None, "a plain column reference is the describe tier's");
        let label = p.head[0].as_ref().unwrap();
        assert!(resolve(label, Some(&[col(false, oid::TEXT), col(false, oid::TEXT)])).non_null);
        assert!(!resolve(label, Some(&[col(false, oid::TEXT), col(true, oid::TEXT)])).non_null);
        assert!(!resolve(label, None).non_null, "no probe, no proof");
        // On SQLite a leaf carries its described type through a function the host registered.
        let host = Trust { total: [("anti_html".to_string(), oid::TEXT)].into_iter().collect(), ..Default::default() };
        let h = plan("SELECT id, anti_html(title) AS title_html FROM todos WHERE owner_id = ?1 ORDER BY title", Dialect::Sqlite, &host);
        assert_eq!(h.leaves.as_deref(), Some("SELECT title AS a0 FROM todos"));
        assert_eq!(resolve(h.head[1].as_ref().unwrap(), Some(&[col(false, oid::TEXT)])), Fact { non_null: true, type_oid: Some(oid::TEXT) });
        // RETURNING reads the written relation.
        let r = pg("UPDATE todos SET done = true WHERE id = $1 RETURNING title || '!' AS shout");
        assert_eq!(r.leaves.as_deref(), Some("SELECT title AS a0 FROM todos"));
    }

    #[test]
    fn a_grouping_set_takes_the_leaves_away() {
        let p = pg("SELECT owner_id || '' AS o, count(*) AS n FROM todos GROUP BY ROLLUP (owner_id)");
        assert_eq!(p.leaves, None);
        assert!(!resolve(p.head[0].as_ref().unwrap(), None).non_null);
        assert!(resolve(p.head[1].as_ref().unwrap(), None).non_null, "a count is still a count");
    }

    #[test]
    fn a_star_splits_the_claims_around_it() {
        let p = pg("SELECT 'a' AS x, t.*, count(*) OVER () AS total FROM todos t");
        assert!(p.star);
        let claims = p.claims(5);
        assert!(claims[0].is_some() && claims[1].is_none() && claims[3].is_none() && claims[4].is_some());
        // A describe with fewer columns than the plan names claims nothing.
        assert!(pg("SELECT 'a', 'b'").claims(3).iter().all(Option::is_none));
    }

    #[test]
    fn a_set_operation_plans_its_arms() {
        let p = pg("WITH d AS (SELECT id FROM todos) SELECT id, title FROM todos WHERE done UNION ALL SELECT id, title FROM todos WHERE id IN (SELECT id FROM d)");
        let set = p.set_op.expect("a set operation");
        assert_eq!(set.kind, SetKind::Union);
        assert_eq!(set.left, "WITH d AS (SELECT id FROM todos) SELECT id, title FROM todos WHERE done");
        assert_eq!(set.right, "WITH d AS (SELECT id FROM todos) SELECT id, title FROM todos WHERE id IN (SELECT id FROM d)");
        assert!(p.head.is_empty());
        // Nested operations nest: the left arm is itself a set operation.
        let n = pg("SELECT 1 UNION SELECT 2 EXCEPT SELECT 3");
        assert_eq!(n.set_op.map(|s| (s.kind, s.left)), Some((SetKind::Except, "SELECT 1 UNION SELECT 2".to_string())));
        assert_eq!(lite("SELECT ?1 AS a INTERSECT SELECT 'x'").set_op.map(|s| s.left), Some("SELECT $1 AS a".to_string()));
    }

    #[test]
    fn set_operation_columns_combine_per_arm() {
        let (nn, nul) = (col(false, oid::UUID), col(true, oid::UUID));
        let facts = |kind, l: &DescribedColumn, r: &DescribedColumn| combine(kind, &[l.clone()], &[r.clone()]).unwrap()[0].non_null;
        assert!(facts(SetKind::Union, &nn, &nn));
        assert!(!facts(SetKind::Union, &nn, &nul), "one nullable arm makes a UNION column nullable");
        assert!(!facts(SetKind::Union, &nul, &nn));
        assert!(facts(SetKind::Intersect, &nul, &nn));
        assert!(!facts(SetKind::Intersect, &nul, &nul));
        assert!(facts(SetKind::Except, &nn, &nul));
        assert!(!facts(SetKind::Except, &nul, &nn));
        assert!(combine(SetKind::Union, &[nn.clone()], &[]).is_none());
    }

    #[test]
    fn a_values_arm_is_non_null_where_every_row_is() {
        let p = pg("VALUES (1, 'a'), (2, NULL)");
        assert!(resolve(p.head[0].as_ref().unwrap(), None).non_null);
        assert!(!resolve(p.head[1].as_ref().unwrap(), None).non_null);
    }

    #[test]
    fn a_row_constructor_is_never_null() {
        let p = pg("SELECT ROW(a, b) AS r, (a, b)::pair AS typed, (a, b) AS bare FROM t");
        for claim in &p.head {
            assert!(resolve(claim.as_ref().unwrap(), None).non_null);
        }
    }

    #[test]
    fn placeholders_are_parameters_and_prove_nothing() {
        assert!(!first(&pg("SELECT $1::uuid AS id")).non_null);
        assert!(!first(&lite("SELECT ?1 || 'x' AS id")).non_null);
        // A host that types its parameters without null says so, and names the exceptions.
        let typed = Trust { non_null_params: true, nullable_params: [2].into_iter().collect(), ..Default::default() };
        let p = plan("SELECT $1::uuid AS id, $2::uuid AS other", Dialect::Postgres, &typed);
        assert!(resolve(p.head[0].as_ref().unwrap(), None).non_null);
        assert!(!resolve(p.head[1].as_ref().unwrap(), None).non_null);
        assert_eq!(dollar_placeholders("SELECT ?1, '?2', \"?3\" -- ?4\n, ?10 /* ?5 */"), "SELECT $1, '?2', \"?3\" -- ?4\n, $10 /* ?5 */");
    }

    #[test]
    fn what_does_not_parse_claims_nothing() {
        assert_eq!(pg("SELEKT 1"), Plan::default());
        assert_eq!(lite("SELECT json_extract('not json', '$') AS boom").head, vec![Some(Expr::Unknown)]);
    }
}
