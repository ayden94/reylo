//! The SQLite analysis, over SQLite's own resolved tree ([`crate::sqlite_tree`], produced by
//! `csrc/lando_sqlite.c` from a patched amalgamation). SQLite resolves the names, the join sides,
//! every parameter's context and the DML target columns itself, and reports each expression's
//! affinity. What SQLite has no facts for stays here, as explicit tables: the result type and
//! nullability of the built-in functions, the type a function gives its arguments, the arithmetic
//! rule, and the row-count proof from the unique keys. The host runs one query for the schema DDL;
//! everything else happens inside the engine.

use crate::api::{Analysis, Column, Diagnostic, Param, Scope};
use crate::dialect::Dialect;
use crate::host::{as_str, Host, Lazy};
use crate::lex::{self, Lexed, Target};
use crate::sqlite_tree::{self as tree, From, Node, Sel, Tree};
use crate::types::Type;
use std::collections::HashSet;

/// The schema, as SQLite stores it; replayed into the engine's in-memory copy.
const DDL_QUERY: &str = "select sql from sqlite_schema where sql is not null and type in ('table', 'view', 'index') order by rowid";

pub fn analyze(sql: &str, lexed: &Lexed, host: &Host) -> Lazy<Analysis> {
    let rows = host.query(DDL_QUERY, Vec::new(), Scope::Session)?;
    let ddl: Vec<String> = match rows {
        Ok(rows) => rows.iter().filter_map(|r| r.first().and_then(as_str)).map(String::from).collect(),
        Err(e) => return Some(crate::postgres::bare(sql, lexed, Dialect::Sqlite, vec![diag(format!("Cannot read the schema: {}", e.message), 0, 1, 5)])),
    };
    // SQLite prepares the `?n` rewrite, not the author's text: its own `:name` ends at the first
    // dot, so `:auth.user_id` survives only as a number. `?n` is the lexer's n-th parameter, which
    // is how `params` matches them back, and an error offset maps back through the rewrite.
    let numbered = lex::rewrite(sql, lexed, Target::Numbered);
    let t = match tree::analyze(&ddl, &numbered.sql) {
        Ok(t) => t,
        Err(message) => return Some(crate::postgres::bare(sql, lexed, Dialect::Sqlite, vec![diag(message, 0, sql.trim_end().len(), 6)])),
    };
    let mut diagnostics = Vec::new();
    if let Some(err) = &t.error {
        let start = usize::try_from(err.offset).map_or(0, |o| numbered.to_original(o)).min(sql.len());
        diagnostics.push(diag(err.message.clone(), start, if err.offset >= 0 { token_len(&sql[start..]) } else { sql.trim_end().len() }, 6));
        // A statement that fails against the replayed schema often fails because a replay step did.
        for e in &t.schema_errors {
            diagnostics.push(diag(format!("The schema replay skipped statement {}: {}", e.index + 1, e.message), 0, 1, 8));
        }
        return Some(crate::postgres::bare(sql, lexed, Dialect::Sqlite, diagnostics));
    }
    let a = Analyzer { tree: &t };
    let (columns, kind) = a.columns();
    let params = a.params(lexed);
    let (cardinality, reason) = match crate::postgres::cardinality_directive(lexed, &mut diagnostics) {
        Some(c) => c,
        None => a.cardinality(),
    };
    Some(Analysis {
        kind,
        params,
        columns,
        cardinality,
        cardinality_reason: reason,
        rewritten: lex::rewrite(sql, lexed, Target::Question),
        style: lexed.style,
        directives: lexed.directives.clone(),
        diagnostics,
        unresolved: Vec::new(),
    })
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
                // A column SQLite could not type but names from a table: its declared affinity.
                let fact = match (&fact.ty, &public.declared) {
                    (Type::Unknown, Some(declared)) => Fact { ty: affinity_type(affinity(declared)), non_null: fact.non_null },
                    _ => fact,
                };
                let type_name = scalar_name(&fact.ty).unwrap_or("").to_string();
                Column { name: public.name.clone(), ty: fact.ty, nullable: !fact.non_null, type_name }
            })
            .collect();
        (columns, kind)
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
                arm.columns.get(i).map(|c| self.type_of(&c.expr, &inner)).unwrap_or_else(Fact::unknown)
            })
            .collect();
        let Some(first) = facts.first() else { return Fact::unknown() };
        let ty = if facts.iter().all(|f| f.ty == first.ty) { first.ty.clone() } else { shared(&facts.iter().collect::<Vec<_>>()) };
        Fact { ty, non_null: facts.iter().all(|f| f.non_null) }
    }

    /// A column reference: through the cursor to a subquery's column, or the table column itself.
    fn column_ref(&self, n: &Node, scope: &ScopeStack) -> Fact {
        for froms in scope.iter().rev() {
            if let Some(f) = froms.iter().find(|f| f.cursor == n.cursor) {
                if let Some(sub) = &f.subquery {
                    let inner = self.select_column(sub, n.index.max(0) as usize, scope);
                    return Fact { ty: inner.ty, non_null: inner.non_null && !f.outer && !n.outer };
                }
                break;
            }
        }
        let aff = n.affinity.as_deref().unwrap_or(&n.aff);
        Fact::new(aff, (n.not_null || n.rowid) && !n.outer)
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
            "variable" => Fact::unknown(),
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
            "neg" | "pos" => {
                let l = arg(&n.left);
                Fact { ty: if is_numeric(&l.ty) { l.ty } else { Type::Scalar { name: "real".into() } }, non_null: l.non_null }
            }
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
                // A scalar subquery: its first column, NULL when it finds no row.
                let ty = n.select.as_deref().map(|s| self.select_column(s, 0, scope).ty).unwrap_or(Type::Unknown);
                Fact { ty, non_null: false }
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
            return match name {
                "count" => done("integer", Nulls::Never),
                "total" => done("real", Nulls::Never),
                "sum" => typed(if is_numeric(&first.ty) { first.ty.clone() } else { Type::Scalar { name: "real".into() } }, Nulls::Always),
                "avg" => done("real", Nulls::Always),
                "min" | "max" => typed(first.ty.clone(), Nulls::Always),
                "group_concat" | "string_agg" => done("text", Nulls::Always),
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
            "upper" | "lower" | "trim" | "ltrim" | "rtrim" | "substr" | "substring" | "replace" | "char" | "printf" | "format" | "concat" | "concat_ws"
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
        if let Some(dml) = &t.dml {
            let scope: ScopeStack = Vec::new();
            if let Some(values) = &dml.values {
                for (i, v) in values.iter().enumerate() {
                    let target = dml.columns.get(i).and_then(|c| c.as_deref()).and_then(|c| self.table_column(&dml.table, c));
                    self.walk(v, Ctx::Typed(target), &scope, &mut found);
                }
            }
            for item in dml.set.iter().chain(dml.upsert.iter().flat_map(|u| u.set.iter())) {
                let target = item.column.as_deref().and_then(|c| self.table_column(&dml.table, c));
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
                Param { name: name.clone(), ty: fact.ty, type_name }
            })
            .collect()
    }

    fn table_column(&self, table: &str, column: &str) -> Option<Fact> {
        let info = self.tree.tables.get(table)?;
        let col = info.columns.iter().find(|c| c.name.eq_ignore_ascii_case(column))?;
        Some(Fact::new(&col.affinity, col.not_null || info.rowid_alias.as_deref().is_some_and(|a| a.eq_ignore_ascii_case(column))))
    }

    fn walk_select(&self, sel: &Sel, scope: &ScopeStack, targets: &[Option<Fact>], found: &mut Vec<(String, Fact)>) {
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
                "neg" | "pos" | "~" | "&" | "|" | "<<" | ">>" => Ctx::Typed(Some(Fact::new("real", true))),
                "and" | "or" | "not" => Ctx::Bool,
                "case" => match slot {
                    // A WHEN compares with the base when there is one, else it is a condition; a
                    // THEN or ELSE shares the other results' type.
                    Slot::List(i) if i % 2 == 0 && i + 1 < n.list.len() => match &n.left {
                        Some(base) => Ctx::Typed(Some(self.type_of(base, scope))),
                        None => Ctx::Bool,
                    },
                    Slot::List(_) => {
                        let others: Vec<Fact> = n.list.iter().enumerate().filter(|(j, _)| j % 2 == 1 || (*j == n.list.len() - 1 && n.list.len() % 2 == 1)).map(|(_, x)| self.type_of(x, scope)).collect();
                        let refs: Vec<&Fact> = others.iter().collect();
                        let ty = shared(&refs);
                        Ctx::Typed(if matches!(ty, Type::Unknown) { None } else { Some(Fact { ty, non_null: true }) })
                    }
                    Slot::Left => Ctx::None,
                    _ => Ctx::None,
                },
                "function" => match slot {
                    Slot::Arg(i) => self.argument(n, i, scope),
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
                        let can_skip = dml.on_error == "ignore" || dml.upsert.as_ref().is_some_and(|u| !u.do_update);
                        if can_skip { ("maybeOne".into(), Some("ON CONFLICT DO NOTHING (or OR IGNORE) can insert no row".into())) } else { ("one".into(), None) }
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
        if sel.prior.is_some() {
            return Err("a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows".into());
        }
        if sel.values {
            return Err("a VALUES list returns a row per row it lists".into());
        }
        let equated = self.equated(sel.where_.as_ref());
        if !sel.group_by.is_empty() {
            // One group at most: the WHERE pins a unique key of a relation to values, and every
            // GROUP BY column belongs to that relation, whatever else is joined.
            for f in &sel.from {
                let Some(table) = f.table.as_deref().filter(|_| f.subquery.is_none() && !f.tab_func) else { continue };
                if !self.key_pinned(table, f.cursor, &equated) {
                    continue;
                }
                if sel.group_by.iter().all(|g| g.op == "column" && g.cursor == f.cursor) {
                    return Ok(());
                }
            }
            return Err("GROUP BY returns one row per group; the engine proves one group only when the WHERE pins a unique key of a relation to values and every GROUP BY column belongs to that relation".into());
        }
        if sel.having.is_some() {
            return Err("HAVING leaves a row per group".into());
        }
        let [only] = sel.from.as_slice() else {
            return Err(match sel.from.len() {
                0 => "a SELECT with no FROM and a WHERE can return no row or one, and the engine proves that only through a LIMIT".into(),
                _ => "a join returns a row per pair it matches, and the engine proves a single row only for one relation".into(),
            });
        };
        let Some(table) = only.table.as_deref().filter(|_| only.subquery.is_none() && !only.tab_func) else {
            return Err("only a plain table in the FROM is matched against a unique key".into());
        };
        if self.key_pinned(table, only.cursor, &equated) {
            return Ok(());
        }
        let keys = self.tree.tables.get(table).map(|t| t.unique_keys.clone()).unwrap_or_default();
        if keys.is_empty() {
            return Err(format!("`{table}` has no unique index the engine can read, so nothing bounds the rows a WHERE matches"));
        }
        let shown: Vec<String> = keys.iter().map(|k| format!("({})", k.join(", "))).collect();
        Err(format!("the WHERE does not equate every column of a unique key of `{table}` — it has {}", shown.join(", ")))
    }

    /// Whether some unique key of `table` (read through `cursor`) has every column equated to a value.
    fn key_pinned(&self, table: &str, cursor: i64, equated: &HashSet<(i64, String)>) -> bool {
        let Some(info) = self.tree.tables.get(table) else { return false };
        info.unique_keys.iter().any(|key| !key.is_empty() && key.iter().all(|c| equated.iter().any(|(cur, name)| *cur == cursor && name.eq_ignore_ascii_case(c))))
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

    /// The columns the WHERE equates to a value: `col = <no column anywhere>` under ANDs.
    fn equated(&self, where_: Option<&Node>) -> HashSet<(i64, String)> {
        let mut out = HashSet::new();
        let mut stack: Vec<&Node> = where_.into_iter().collect();
        while let Some(n) = stack.pop() {
            match n.op.as_str() {
                "and" => {
                    stack.extend(n.left.as_deref());
                    stack.extend(n.right.as_deref());
                }
                "=" => {
                    if let (Some(l), Some(r)) = (n.left.as_deref(), n.right.as_deref()) {
                        for (col, other) in [(l, r), (r, l)] {
                            if col.op == "column" && !reads_column(other) {
                                if let Some(name) = &col.column {
                                    out.insert((col.cursor, name.clone()));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }
}

/// The type of `a op b` for `+`, `-`, `*`, `/`, `%`: integer stays integer, anything else is real.
fn arithmetic(l: &Type, r: &Type) -> Type {
    match (scalar_name(l), scalar_name(r)) {
        (Some("integer"), Some("integer")) => Type::Scalar { name: "integer".into() },
        _ => Type::Scalar { name: "real".into() },
    }
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
enum Ctx {
    Typed(Option<Fact>),
    Bool,
    None,
}

enum Slot {
    Left,
    Right,
    List(usize),
    Arg(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(sql: &str) -> Tree {
        let ddl = vec![
            "create table users (id integer primary key, name text not null, email text unique, score real, avatar blob, created_at text not null default current_timestamp)".to_string(),
            "create table posts (id integer primary key, user_id integer not null references users(id), title text not null, body text)".to_string(),
        ];
        tree::analyze(&ddl, sql).unwrap()
    }

    fn analysis(sql: &str) -> Analysis {
        let lexed = lex::lex(sql, Dialect::Sqlite).unwrap();
        let answers = std::collections::HashMap::from([(
            format!("query:{DDL_QUERY}|[]"),
            crate::api::Answer::Rows { rows: tree("select 1").tables.keys().map(|_| Vec::new()).collect() },
        )]);
        let _ = answers;
        let t = tree(&lex::rewrite(sql, &lexed, Target::Numbered).sql);
        let a = Analyzer { tree: &t };
        let (columns, kind) = a.columns();
        let params = a.params(&lexed);
        let (cardinality, reason) = a.cardinality();
        Analysis { kind, params, columns, cardinality, cardinality_reason: reason, rewritten: lex::rewrite(sql, &lexed, Target::Question), style: lexed.style, directives: vec![], diagnostics: vec![], unresolved: vec![] }
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
        // Positional forms: SQLite's `?3` is the lexer's `$3`, and a bare `?` its `?1`.
        assert_eq!(param(&analyzed("select name from users where id = ?3"), "$3"), "integer");
        assert_eq!(param(&analyzed("select name from users where email = ?"), "?1"), "text");
        // An error SQLite reports in the rewrite lands on the author's text.
        let sql = "select :auth.user_id, nope from users";
        let a = analyzed(sql);
        assert_eq!(a.diagnostics.len(), 1, "{:?}", a.diagnostics);
        assert_eq!((a.diagnostics[0].start, a.diagnostics[0].length), (sql.find("nope").unwrap(), 4));
    }

    #[test]
    fn sqlite_only_syntax_parses() {
        let a = analysis("select id from users limit :n, :m");
        assert_eq!(param(&a, "n"), "integer");
        assert_eq!(analysis("insert or replace into users (id, name) values (:id, :name) returning id").cardinality, "one");
    }
}
