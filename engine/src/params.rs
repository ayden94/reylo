//! Parameter types read off the statement, for a database that cannot describe one.
//!
//! SQLite binds whatever it is given and types nothing; MySQL's server guesses. But a placeholder
//! sits in a context that names its type: `where id = ?` is the type of `id`, `cast(? as integer)`
//! is an integer, `limit ?` is an integer, `name like ?` is text, the third value of an `INSERT`
//! is the third column's type, `set score = ?` is `score`'s. This pass walks the parse tree with
//! the tables in scope and answers a type per placeholder where one of those contexts holds, and
//! nothing where none does — a parameter under a function call proves nothing.

use pg_query::protobuf::AExprKind;
use serde_json::Value;
use std::collections::HashMap;

/// A column as the catalog describes it: the declared type is the dialect's own spelling
/// (`INTEGER`, `varchar(255)`), read by the caller's affinity rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnInfo {
    pub name: String,
    pub declared: String,
    pub not_null: bool,
    pub pk: bool,
}

/// Table name → its columns in attribute order.
pub type Tables = HashMap<String, Vec<ColumnInfo>>;

/// What the context said about one placeholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hint {
    /// The declared type of a column the placeholder is compared with or assigned to.
    Column(String),
    /// A cast target, by the name the parser gives it (`int4`, `text`, `float8`, `numeric`, `bytea`).
    Cast(String),
    Integer,
    Real,
    Text,
}

/// Every table name a statement mentions, so the caller can fetch their columns first.
pub fn tables_mentioned(tree: &Value) -> Vec<String> {
    let mut out = Vec::new();
    visit(tree, &mut |key, node| {
        if key == "RangeVar" {
            if let Some(rel) = node.get("relname").and_then(Value::as_str) {
                if !out.iter().any(|r| r == rel) {
                    out.push(rel.to_string());
                }
            }
        }
    });
    out
}

/// A type per result column of the top statement's select list (or `RETURNING`), where the
/// SQL states one: a column's declared type, a literal's kind, a cast, arithmetic over a typed
/// operand, a `||`. `None` where it does not. Used for a database that types no expression.
pub fn result_types(tree: &Value, tables: &Tables) -> Vec<Option<Hint>> {
    let Some(stmt) = tree.pointer("/stmts/0/stmt") else { return Vec::new() };
    let Some(node) = stmt.get("node").and_then(Value::as_object) else { return Vec::new() };
    let Some((kind, body)) = node.iter().next() else { return Vec::new() };
    let mut out = HashMap::new();
    let mut walker = Walker { tables, hints: &mut out, scope: Vec::new() };
    let (targets, from) = match kind.as_str() {
        "SelectStmt" => (body.get("target_list"), body.get("from_clause")),
        "InsertStmt" | "UpdateStmt" | "DeleteStmt" => {
            walker.push_relation(body.get("relation"));
            (body.get("returning_list"), body.get("from_clause").or(body.get("using_clause")))
        }
        _ => return Vec::new(),
    };
    walker.push_from(from);
    targets
        .and_then(Value::as_array)
        .map(|ts| ts.iter().map(|t| t.pointer("/node/ResTarget/val").and_then(|v| walker.type_of(v))).collect())
        .unwrap_or_default()
}

/// A type per placeholder number, where the statement names one.
pub fn infer(tree: &Value, tables: &Tables) -> HashMap<i32, Hint> {
    let mut out = HashMap::new();
    let Some(stmt) = tree.pointer("/stmts/0/stmt") else { return out };
    let mut walker = Walker { tables, hints: &mut out, scope: Vec::new() };
    walker.statement(stmt);
    out
}

struct Walker<'a> {
    tables: &'a Tables,
    hints: &'a mut HashMap<i32, Hint>,
    /// (alias or table name, table name), innermost last.
    scope: Vec<(String, String)>,
}

impl Walker<'_> {
    fn hint(&mut self, n: i32, hint: Hint) {
        self.hints.entry(n).or_insert(hint);
    }

    fn statement(&mut self, wrapper: &Value) {
        let Some(node) = wrapper.get("node").and_then(Value::as_object) else { return };
        let Some((kind, body)) = node.iter().next() else { return };
        match kind.as_str() {
            "SelectStmt" => self.select(body),
            "InsertStmt" => self.insert(body),
            "UpdateStmt" => self.update(body),
            "DeleteStmt" => {
                let mark = self.scope.len();
                self.push_relation(body.get("relation"));
                self.push_from(body.get("using_clause"));
                self.expr(body.get("where_clause"));
                self.expr(body.get("returning_list"));
                self.statement(body.get("with_clause").unwrap_or(&Value::Null));
                self.scope.truncate(mark);
            }
            _ => self.expr(Some(wrapper)),
        }
    }

    fn select(&mut self, s: &Value) {
        let mark = self.scope.len();
        if let Some(with) = s.get("with_clause").and_then(|w| w.get("ctes")).and_then(Value::as_array) {
            for cte in with {
                if let Some(q) = cte.pointer("/node/CommonTableExpr/ctequery") {
                    self.statement(q);
                }
            }
        }
        for arm in ["larg", "rarg"] {
            if let Some(a) = s.get(arm).filter(|a| !a.is_null()) {
                self.select(a);
            }
        }
        self.push_from(s.get("from_clause"));
        self.expr(s.get("target_list"));
        self.expr(s.get("where_clause"));
        self.expr(s.get("group_clause"));
        self.expr(s.get("having_clause"));
        self.expr(s.get("sort_clause"));
        for limit in ["limit_count", "limit_offset"] {
            if let Some(n) = s.get(limit).and_then(param_number) {
                self.hint(n, Hint::Integer);
            } else {
                self.expr(s.get(limit));
            }
        }
        self.expr(s.get("values_lists"));
        self.scope.truncate(mark);
    }

    fn insert(&mut self, i: &Value) {
        let mark = self.scope.len();
        self.push_relation(i.get("relation"));
        let table = i.pointer("/relation/relname").and_then(Value::as_str).unwrap_or("").to_string();
        let named: Vec<String> = i
            .get("cols")
            .and_then(Value::as_array)
            .map(|cols| cols.iter().filter_map(|c| c.pointer("/node/ResTarget/name").and_then(Value::as_str).map(String::from)).collect())
            .unwrap_or_default();
        let columns: Vec<String> = if named.is_empty() {
            self.tables.get(&table).map(|cs| cs.iter().map(|c| c.name.clone()).collect()).unwrap_or_default()
        } else {
            named
        };
        if let Some(select) = i.pointer("/select_stmt/node/SelectStmt") {
            let rows = select.get("values_lists").and_then(Value::as_array);
            match rows {
                Some(rows) if !rows.is_empty() => {
                    for row in rows {
                        let Some(items) = row.pointer("/node/List/items").and_then(Value::as_array) else { continue };
                        for (j, item) in items.iter().enumerate() {
                            self.value_for_column(&table, columns.get(j).map(String::as_str), item);
                        }
                    }
                }
                _ => {
                    // `INSERT … SELECT`: a placeholder in the select list takes the column it feeds.
                    if let Some(targets) = select.get("target_list").and_then(Value::as_array) {
                        for (j, t) in targets.iter().enumerate() {
                            if let Some(val) = t.pointer("/node/ResTarget/val") {
                                self.value_for_column(&table, columns.get(j).map(String::as_str), val);
                            }
                        }
                    }
                    self.select(select);
                }
            }
        }
        if let Some(on_conflict) = i.get("on_conflict_clause").filter(|c| !c.is_null()) {
            for t in on_conflict.get("target_list").and_then(Value::as_array).into_iter().flatten() {
                let name = t.pointer("/node/ResTarget/name").and_then(Value::as_str);
                if let Some(val) = t.pointer("/node/ResTarget/val") {
                    self.value_for_column(&table, name, val);
                }
            }
            self.expr(on_conflict.get("where_clause"));
        }
        self.expr(i.get("returning_list"));
        self.scope.truncate(mark);
    }

    fn update(&mut self, u: &Value) {
        let mark = self.scope.len();
        self.push_relation(u.get("relation"));
        self.push_from(u.get("from_clause"));
        let table = u.pointer("/relation/relname").and_then(Value::as_str).unwrap_or("").to_string();
        for t in u.get("target_list").and_then(Value::as_array).into_iter().flatten() {
            let name = t.pointer("/node/ResTarget/name").and_then(Value::as_str);
            if let Some(val) = t.pointer("/node/ResTarget/val") {
                self.value_for_column(&table, name, val);
            }
        }
        self.expr(u.get("where_clause"));
        self.expr(u.get("returning_list"));
        self.scope.truncate(mark);
    }

    /// A value assigned to `column` of `table`: a placeholder takes the column's type; anything
    /// else is walked as an expression.
    fn value_for_column(&mut self, table: &str, column: Option<&str>, val: &Value) {
        match (param_number(val), column.and_then(|c| self.declared_of(table, c))) {
            (Some(n), Some(declared)) => self.hint(n, Hint::Column(declared)),
            _ => self.expr(Some(val)),
        }
    }

    fn push_relation(&mut self, rv: Option<&Value>) {
        let Some(rv) = rv.filter(|r| !r.is_null()) else { return };
        let Some(name) = rv.get("relname").and_then(Value::as_str) else { return };
        let alias = rv.pointer("/alias/aliasname").and_then(Value::as_str).unwrap_or(name);
        self.scope.push((alias.to_string(), name.to_string()));
    }

    fn push_from(&mut self, from: Option<&Value>) {
        let Some(items) = from.and_then(Value::as_array) else { return };
        for item in items {
            self.push_from_item(item);
        }
    }

    fn push_from_item(&mut self, item: &Value) {
        if let Some(rv) = item.pointer("/node/RangeVar") {
            self.push_relation(Some(rv));
        } else if let Some(j) = item.pointer("/node/JoinExpr") {
            self.push_from_item(j.get("larg").unwrap_or(&Value::Null));
            self.push_from_item(j.get("rarg").unwrap_or(&Value::Null));
            self.expr(j.get("quals"));
        }
        // A subselect in FROM has its own columns; nothing here reads them.
    }

    fn declared_of(&self, table: &str, column: &str) -> Option<String> {
        self.tables.get(table)?.iter().find(|c| c.name.eq_ignore_ascii_case(column)).map(|c| c.declared.clone())
    }

    /// The declared type of a column reference, through the scope: `alias.col`, or a bare `col`
    /// found in the innermost relation that has it.
    fn column_type(&self, expr: &Value) -> Option<String> {
        let cr = expr.pointer("/node/ColumnRef")?;
        let parts: Vec<&str> = cr.get("fields")?.as_array()?.iter().filter_map(|f| f.pointer("/node/String/sval").and_then(Value::as_str)).collect();
        match parts.as_slice() {
            [col] => self.scope.iter().rev().find_map(|(_, table)| self.declared_of(table, col)),
            [qual, col] => self.scope.iter().rev().find(|(alias, _)| alias == qual).and_then(|(_, table)| self.declared_of(table, col)),
            _ => None,
        }
    }

    /// What an expression's type is, when the statement states it: a column, a literal, a cast.
    fn type_of(&self, expr: &Value) -> Option<Hint> {
        if let Some(declared) = self.column_type(expr) {
            return Some(Hint::Column(declared));
        }
        if let Some(c) = expr.pointer("/node/AConst") {
            let val = c.get("val")?;
            return if val.get("Ival").is_some() {
                Some(Hint::Integer)
            } else if val.get("Fval").is_some() {
                Some(Hint::Real)
            } else if val.get("Sval").is_some() {
                Some(Hint::Text)
            } else {
                None
            };
        }
        if let Some(t) = expr.pointer("/node/TypeCast") {
            return cast_name(t).map(Hint::Cast);
        }
        if let Some(a) = expr.pointer("/node/AExpr") {
            if a.get("kind").and_then(Value::as_i64) == Some(AExprKind::AexprOp as i64) {
                let op = last_string(a.get("name"))?;
                if op == "||" {
                    return Some(Hint::Text);
                }
                if ["+", "-", "*", "/", "%"].contains(&op.as_str()) {
                    return a.get("lexpr").and_then(|l| self.type_of(l)).or_else(|| a.get("rexpr").and_then(|r| self.type_of(r)));
                }
            }
        }
        if let Some(c) = expr.pointer("/node/CoalesceExpr") {
            return c.get("args")?.as_array()?.iter().find_map(|a| self.type_of(a));
        }
        if let Some(f) = expr.pointer("/node/FuncCall") {
            let name = last_string(f.get("funcname"))?;
            let args: Vec<&Value> = f.get("args").and_then(Value::as_array).map(|a| a.iter().collect()).unwrap_or_default();
            return match name.as_str() {
                "count" | "length" | "instr" | "unixepoch" | "random" | "changes" | "last_insert_rowid" => Some(Hint::Integer),
                "lower" | "upper" | "trim" | "ltrim" | "rtrim" | "substr" | "substring" | "replace" | "hex" | "quote" | "typeof" | "date" | "time" | "datetime" | "strftime" | "group_concat" | "string_agg" | "printf" | "format" | "json" | "json_object" | "json_array" | "json_group_array" | "json_group_object" | "json_extract" => Some(Hint::Text),
                "julianday" | "total" | "avg" | "round" => Some(Hint::Real),
                "sum" | "min" | "max" | "abs" | "ifnull" | "nullif" | "iif" => args.iter().find_map(|a| self.type_of(a)),
                _ => None,
            };
        }
        None
    }

    /// Walk an expression (or a list of them), reading every placeholder's context.
    fn expr(&mut self, value: Option<&Value>) {
        let Some(value) = value else { return };
        match value {
            Value::Array(items) => {
                for item in items {
                    self.expr(Some(item));
                }
            }
            Value::Object(_) => {
                let Some(node) = value.get("node").and_then(Value::as_object) else {
                    // A bare protobuf message rather than a wrapped node: walk its children.
                    for (_, child) in value.as_object().unwrap() {
                        self.expr(Some(child));
                    }
                    return;
                };
                let Some((kind, body)) = node.iter().next() else { return };
                self.node(kind, body);
            }
            _ => {}
        }
    }

    fn node(&mut self, kind: &str, body: &Value) {
        match kind {
            "AExpr" => self.aexpr(body),
            "TypeCast" => {
                if let (Some(n), Some(name)) = (body.get("arg").and_then(param_number), cast_name(body)) {
                    self.hint(n, Hint::Cast(name));
                } else {
                    self.expr(body.get("arg"));
                }
            }
            "CoalesceExpr" => {
                let args: Vec<Value> = body.get("args").and_then(Value::as_array).cloned().unwrap_or_default();
                let known = args.iter().find_map(|a| self.type_of(a));
                for a in &args {
                    match (param_number(a), &known) {
                        (Some(n), Some(hint)) => self.hint(n, hint.clone()),
                        _ => self.expr(Some(a)),
                    }
                }
            }
            "SubLink" => {
                self.expr(body.get("testexpr"));
                if let Some(sub) = body.get("subselect") {
                    self.statement(sub);
                }
            }
            "SelectStmt" => self.select(body),
            "ResTarget" => self.expr(body.get("val")),
            "FuncCall" => {
                // A handful of SQLite's own functions take text: the date and time family.
                let name = last_string(body.get("funcname")).unwrap_or_default();
                let text_fns = ["date", "time", "datetime", "julianday", "unixepoch", "strftime", "lower", "upper", "trim", "ltrim", "rtrim", "length", "instr", "substr", "substring", "replace", "hex", "quote", "like", "glob"];
                let args: Vec<Value> = body.get("args").and_then(Value::as_array).cloned().unwrap_or_default();
                for (i, a) in args.iter().enumerate() {
                    match param_number(a) {
                        Some(n) if text_fns.contains(&name.as_str()) && (i == 0 || name == "replace" || name == "like" || name == "glob" || name == "instr") => self.hint(n, Hint::Text),
                        _ => self.expr(Some(a)),
                    }
                }
                self.expr(body.get("agg_filter"));
                self.expr(body.get("over"));
            }
            _ => {
                for (_, child) in body.as_object().into_iter().flatten() {
                    self.expr(Some(child));
                }
            }
        }
    }

    fn aexpr(&mut self, a: &Value) {
        let kind = a.get("kind").and_then(Value::as_i64).unwrap_or(0);
        let (lexpr, rexpr) = (a.get("lexpr"), a.get("rexpr"));
        let op = last_string(a.get("name")).unwrap_or_default();
        let is = |k: AExprKind| kind == k as i64;
        if is(AExprKind::AexprLike) || is(AExprKind::AexprIlike) || is(AExprKind::AexprSimilar) {
            for side in [lexpr, rexpr].into_iter().flatten() {
                match param_number(side) {
                    Some(n) => self.hint(n, Hint::Text),
                    None => self.expr(Some(side)),
                }
            }
            return;
        }
        if is(AExprKind::AexprIn) || is(AExprKind::AexprBetween) || is(AExprKind::AexprNotBetween) || is(AExprKind::AexprBetweenSym) || is(AExprKind::AexprNotBetweenSym) {
            let subject = lexpr.and_then(|l| self.type_of(l));
            self.expr(lexpr);
            let items: Vec<Value> = rexpr.and_then(|r| r.pointer("/node/List/items")).and_then(Value::as_array).cloned().unwrap_or_default();
            for item in &items {
                match (param_number(item), &subject) {
                    (Some(n), Some(hint)) => self.hint(n, hint.clone()),
                    _ => self.expr(Some(item)),
                }
            }
            if let (Some(n), Some(hint)) = (lexpr.and_then(param_number), items.iter().find_map(|i| self.type_of(i))) {
                self.hint(n, hint);
            }
            return;
        }
        if is(AExprKind::AexprOp) || is(AExprKind::AexprDistinct) || is(AExprKind::AexprNotDistinct) || is(AExprKind::AexprNullif) {
            if op == "||" {
                for side in [lexpr, rexpr].into_iter().flatten() {
                    match param_number(side) {
                        Some(n) => self.hint(n, Hint::Text),
                        None => self.expr(Some(side)),
                    }
                }
                return;
            }
            let (lp, rp) = (lexpr.and_then(param_number), rexpr.and_then(param_number));
            let (lt, rt) = (lexpr.and_then(|l| self.type_of(l)), rexpr.and_then(|r| self.type_of(r)));
            if let (Some(n), Some(hint)) = (lp, rt.clone()) {
                self.hint(n, hint);
            }
            if let (Some(n), Some(hint)) = (rp, lt) {
                self.hint(n, hint);
            }
            if lp.is_none() {
                self.expr(lexpr);
            }
            if rp.is_none() {
                self.expr(rexpr);
            }
            return;
        }
        self.expr(lexpr);
        self.expr(rexpr);
    }
}

/// A wrapped node that is a placeholder, through casts.
fn param_number(wrapper: &Value) -> Option<i32> {
    let inner = wrapper.get("node")?;
    if let Some(p) = inner.get("ParamRef") {
        return p.get("number").and_then(Value::as_i64).map(|n| n as i32);
    }
    None
}

fn cast_name(t: &Value) -> Option<String> {
    last_string(t.pointer("/type_name/names"))
}

fn last_string(names: Option<&Value>) -> Option<String> {
    names?.as_array()?.last()?.pointer("/node/String/sval")?.as_str().map(String::from)
}

fn visit(v: &Value, f: &mut dyn FnMut(&str, &Value)) {
    match v {
        Value::Object(map) => {
            for (k, val) in map {
                f(k, val);
                visit(val, f);
            }
        }
        Value::Array(items) => {
            for item in items {
                visit(item, f);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tables() -> Tables {
        let mut t = Tables::new();
        t.insert(
            "users".into(),
            vec![
                ColumnInfo { name: "id".into(), declared: "INTEGER".into(), not_null: true, pk: true },
                ColumnInfo { name: "name".into(), declared: "TEXT".into(), not_null: true, pk: false },
                ColumnInfo { name: "score".into(), declared: "REAL".into(), not_null: false, pk: false },
            ],
        );
        t.insert("posts".into(), vec![ColumnInfo { name: "user_id".into(), declared: "INTEGER".into(), not_null: true, pk: false }, ColumnInfo { name: "title".into(), declared: "TEXT".into(), not_null: true, pk: false }]);
        t
    }

    fn hints(sql: &str) -> HashMap<i32, Hint> {
        let tree = serde_json::to_value(&pg_query::parse(sql).unwrap().protobuf).unwrap();
        infer(&tree, &tables())
    }

    #[test]
    fn comparisons_casts_limits() {
        let h = hints("select * from users u where u.id = $1 and name like $2 and score > $3 limit $4 offset $5");
        assert_eq!(h[&1], Hint::Column("INTEGER".into()));
        assert_eq!(h[&2], Hint::Text);
        assert_eq!(h[&3], Hint::Column("REAL".into()));
        assert_eq!(h[&4], Hint::Integer);
        assert_eq!(h[&5], Hint::Integer);
        let h = hints("select cast($1 as integer), $2 || 'x', 1 + $3 from users where id in ($4, $5) and score between $6 and $7");
        assert_eq!(h[&1], Hint::Cast("int4".into()));
        assert_eq!(h[&2], Hint::Text);
        assert_eq!(h[&3], Hint::Integer);
        assert_eq!(h[&4], Hint::Column("INTEGER".into()));
        assert_eq!(h[&5], Hint::Column("INTEGER".into()));
        assert_eq!(h[&6], Hint::Column("REAL".into()));
        assert_eq!(h[&7], Hint::Column("REAL".into()));
    }

    #[test]
    fn dml_positions() {
        let h = hints("insert into users (name, score) values ($1, $2)");
        assert_eq!(h[&1], Hint::Column("TEXT".into()));
        assert_eq!(h[&2], Hint::Column("REAL".into()));
        let h = hints("insert into users values ($1, $2, $3)");
        assert_eq!(h[&3], Hint::Column("REAL".into()));
        let h = hints("update users set score = $1, name = coalesce($2, name) where id = $3");
        assert_eq!(h[&1], Hint::Column("REAL".into()));
        assert_eq!(h[&2], Hint::Column("TEXT".into()));
        assert_eq!(h[&3], Hint::Column("INTEGER".into()));
    }

    #[test]
    fn joins_and_subqueries() {
        let h = hints("select u.name from users u join posts p on p.user_id = u.id where p.title = $1 and exists (select 1 from posts q where q.user_id = $2)");
        assert_eq!(h[&1], Hint::Column("TEXT".into()));
        assert_eq!(h[&2], Hint::Column("INTEGER".into()));
        let h = hints("select length($1), abs($2) from users");
        assert_eq!(h[&1], Hint::Text);
        assert!(!h.contains_key(&2));
    }
}
