//! Statement analysis over the real parse tree (libpg_query). Statement kind, locking,
//! read/write sets, called functions, and the binding-site analysis that reads optionality
//! and defaults out of the SQL itself (`coalesce`, null guards, the input-CTE convention).
//!
//! The parse tree is walked as generic JSON (the protobuf types serialize), so every
//! clause — including LIMIT, VALUES, and ON CONFLICT — is reached; the crate's own typed
//! traversal covers only a subset of node types.

use pg_query::protobuf::{self, node::Node as NodeEnum};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StatementKind {
    Query,
    Mutation,
}

#[derive(Debug, Clone, Default)]
pub struct ParamUse {
    /// Optional because a `coalesce(:p, …)` wraps a use; carries the literal default when one exists.
    pub coalesce: bool,
    pub coalesce_default: Option<Value>,
    /// Optional+nullable because a use is null-guarded (`:p IS NULL OR …`).
    pub null_guarded: bool,
}

#[derive(Debug, Clone)]
pub struct Analysis {
    pub kind: StatementKind,
    /// `select`, `insert`, `update`, `delete` or `merge`.
    pub statement: &'static str,
    pub has_locking: bool,
    /// Tables read (SELECT contexts).
    pub read_set: Vec<String>,
    /// Tables written (DML contexts).
    pub write_set: Vec<String>,
    /// Function names called anywhere in the statement.
    pub functions: Vec<String>,
    /// Binding-site info per parameter number (1-based).
    pub params: HashMap<i32, ParamUse>,
    /// Whether RETURNING produces rows (for defaulting `returns`).
    pub has_returning: bool,
    /// Nullability degradation triggers: an outer join anywhere, a repeated table, a set op.
    pub has_outer_join: bool,
    /// Tables on the nullable side of an outer join (the right side of a LEFT JOIN, the left
    /// of a RIGHT JOIN, both of a FULL JOIN): their columns may be null on the result.
    pub nullable_tables: Vec<String>,
    pub has_repeated_table: bool,
    pub has_set_op: bool,
    /// `GROUP BY ROLLUP`, `CUBE`, or `GROUPING SETS` anywhere: a grouping column is NULL in the
    /// rows that aggregate over it, whatever the catalog says about the column.
    pub has_grouping_sets: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyzeError {
    Parse(String),
    NotOneStatement(usize),
    UnsupportedStatement,
}

impl std::fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzeError::Parse(m) => write!(f, "SQL does not parse: {m}"),
            AnalyzeError::NotOneStatement(n) => write!(f, "expected exactly one statement, found {n}"),
            AnalyzeError::UnsupportedStatement => f.write_str("only SELECT, INSERT, UPDATE, DELETE and MERGE statements are typed"),
        }
    }
}

/// Analyze one rewritten statement (placeholders already `$n`).
pub fn analyze(sql: &str) -> Result<Analysis, AnalyzeError> {
    let parsed = pg_query::parse(sql).map_err(|e| AnalyzeError::Parse(clean_parse_error(&e.to_string())))?;
    if parsed.protobuf.stmts.len() != 1 {
        return Err(AnalyzeError::NotOneStatement(parsed.protobuf.stmts.len()));
    }
    let top = parsed.protobuf.stmts[0]
        .stmt
        .as_ref()
        .and_then(|s| s.node.as_ref())
        .ok_or(AnalyzeError::UnsupportedStatement)?;
    let (top_kind, statement) = match top {
        NodeEnum::SelectStmt(_) => (StatementKind::Query, "select"),
        NodeEnum::InsertStmt(_) => (StatementKind::Mutation, "insert"),
        NodeEnum::UpdateStmt(_) => (StatementKind::Mutation, "update"),
        NodeEnum::DeleteStmt(_) => (StatementKind::Mutation, "delete"),
        NodeEnum::MergeStmt(_) => (StatementKind::Mutation, "merge"),
        _ => return Err(AnalyzeError::UnsupportedStatement),
    };

    let tree = serde_json::to_value(&parsed.protobuf).map_err(|e| AnalyzeError::Parse(e.to_string()))?;

    // Pass 1: CTE columns that are direct parameter bindings — (cte, column) -> param number.
    let mut cte_param_columns: HashMap<(String, String), i32> = HashMap::new();
    visit(&tree, &mut |key, node| {
        if key != "CommonTableExpr" {
            return;
        }
        let Some(ctename) = node.get("ctename").and_then(|v| v.as_str()) else { return };
        let Some(select) = node.get("ctequery").and_then(|q| q.get("node")).and_then(|n| n.get("SelectStmt")) else {
            return;
        };
        let Some(targets) = select.get("target_list").and_then(|t| t.as_array()) else { return };
        for target in targets {
            let Some(rt) = target.get("node").and_then(|n| n.get("ResTarget")) else { continue };
            let Some(name) = rt.get("name").and_then(|n| n.as_str()).filter(|n| !n.is_empty()) else { continue };
            if let Some(n) = rt.get("val").and_then(direct_param_number) {
                cte_param_columns.insert((ctename.to_string(), name.to_string()), n);
            }
        }
    });

    // Pass 2: everything else.
    let mut kind = top_kind;
    let mut has_locking = false;
    let mut has_returning = false;
    let mut has_outer_join = false;
    let mut has_set_op = false;
    let mut has_repeated_table = false;
    let mut has_grouping_sets = false;
    let mut nullable_tables: Vec<String> = Vec::new();
    let mut relnames: Vec<String> = Vec::new();
    let mut functions: Vec<String> = Vec::new();
    let mut params: HashMap<i32, ParamUse> = HashMap::new();

    let is_null = protobuf::NullTestType::IsNull as i32;
    let outer_joins = [
        protobuf::JoinType::JoinLeft as i32,
        protobuf::JoinType::JoinRight as i32,
        protobuf::JoinType::JoinFull as i32,
    ];
    let setop_none = protobuf::SetOperation::SetopNone as i32;

    visit(&tree, &mut |key, node| match key {
        "SelectStmt" => {
            if node.get("locking_clause").and_then(|l| l.as_array()).is_some_and(|l| !l.is_empty()) {
                has_locking = true;
            }
            if let Some(op) = node.get("op").and_then(|o| o.as_i64()) {
                if op != setop_none as i64 && op != 0 {
                    has_set_op = true;
                }
            }
        }
        "GroupingSet" => has_grouping_sets = true,
        "InsertStmt" | "UpdateStmt" | "DeleteStmt" | "MergeStmt" => {
            kind = StatementKind::Mutation;
            if node.get("returning_list").and_then(|l| l.as_array()).is_some_and(|l| !l.is_empty()) {
                has_returning = true;
            }
        }
        "JoinExpr" => {
            if let Some(jt) = node.get("jointype").and_then(|j| j.as_i64()) {
                let jt = jt as i32;
                if outer_joins.contains(&jt) {
                    has_outer_join = true;
                }
                let mut mark = |side: &str| {
                    if let Some(sub) = node.get(side) {
                        for rel in relnames_in(sub) {
                            if !nullable_tables.contains(&rel) {
                                nullable_tables.push(rel);
                            }
                        }
                    }
                };
                if jt == protobuf::JoinType::JoinLeft as i32 || jt == protobuf::JoinType::JoinFull as i32 {
                    mark("rarg");
                }
                if jt == protobuf::JoinType::JoinRight as i32 || jt == protobuf::JoinType::JoinFull as i32 {
                    mark("larg");
                }
            }
        }
        "RangeVar" => {
            if let Some(rel) = node.get("relname").and_then(|r| r.as_str()) {
                if relnames.iter().any(|r| r == rel) {
                    has_repeated_table = true;
                } else {
                    relnames.push(rel.to_string());
                }
            }
        }
        "FuncCall" => {
            let name = node
                .get("funcname")
                .and_then(|f| f.as_array())
                .and_then(|parts| parts.last())
                .and_then(|p| p.get("node"))
                .and_then(|n| n.get("String"))
                .and_then(|s| s.get("sval"))
                .and_then(|s| s.as_str());
            if let Some(name) = name {
                if !functions.iter().any(|f| f == name) {
                    functions.push(name.to_string());
                }
            }
        }
        "CoalesceExpr" => {
            let Some(args) = node.get("args").and_then(|a| a.as_array()) else { return };
            if let Some(n) = args.first().and_then(direct_param_number) {
                let entry = params.entry(n).or_default();
                entry.coalesce = true;
                if let Some(d) = args.get(1).and_then(const_json) {
                    entry.coalesce_default = Some(d);
                }
            }
        }
        "NullTest" => {
            if node.get("nulltesttype").and_then(|t| t.as_i64()) != Some(is_null as i64) {
                return;
            }
            let Some(arg) = node.get("arg") else { return };
            let hit = direct_param_number(arg).or_else(|| {
                column_ref_fields(arg).and_then(|(qual, col)| cte_param_columns.get(&(qual?, col)).copied())
            });
            if let Some(n) = hit {
                params.entry(n).or_default().null_guarded = true;
            }
        }
        _ => {}
    });

    let read_set = parsed.select_tables();
    let write_set = parsed.dml_tables();

    Ok(Analysis {
        kind,
        statement,
        has_locking,
        read_set,
        write_set,
        functions,
        params,
        has_returning,
        has_outer_join,
        nullable_tables,
        has_repeated_table,
        has_set_op,
        has_grouping_sets,
    })
}

/// The parameter numbers a Postgres error at `position` (the 1-based character offset the
/// server reports) is about: the parameter standing at that spot, the parameter under a cast
/// there, or the direct parameter operands of the operator or the function call there —
/// which is where "operator does not exist: text = integer" and "function f(integer) does not
/// exist" point. Empty when nothing bound sits at the position.
pub fn params_at_position(sql: &str, position: usize) -> Vec<i32> {
    let Some(offset) = sql.char_indices().nth(position.saturating_sub(1)).map(|(b, _)| b as i64) else {
        return Vec::new();
    };
    let Ok(parsed) = pg_query::parse(sql) else { return Vec::new() };
    let Ok(tree) = serde_json::to_value(&parsed.protobuf) else { return Vec::new() };
    let mut out: Vec<i32> = Vec::new();
    let mut push = |n: Option<i32>| {
        if let Some(n) = n {
            if !out.contains(&n) {
                out.push(n);
            }
        }
    };
    visit(&tree, &mut |key, node| {
        if node.get("location").and_then(|l| l.as_i64()) != Some(offset) {
            return;
        }
        match key {
            "ParamRef" => push(node.get("number").and_then(|n| n.as_i64()).map(|n| n as i32)),
            "TypeCast" => push(node.get("arg").and_then(direct_param_number)),
            "AExpr" => {
                push(node.get("lexpr").and_then(direct_param_number));
                push(node.get("rexpr").and_then(direct_param_number));
            }
            "FuncCall" => {
                if let Some(args) = node.get("args").and_then(|a| a.as_array()) {
                    for arg in args {
                        push(direct_param_number(arg));
                    }
                }
            }
            _ => {}
        }
    });
    out
}

/// Every table name referenced below a node.
fn relnames_in(v: &Value) -> Vec<String> {
    let mut out = Vec::new();
    visit(v, &mut |key, node| {
        if key == "RangeVar" {
            if let Some(rel) = node.get("relname").and_then(|r| r.as_str()) {
                out.push(rel.to_string());
            }
        }
    });
    out
}

/// Generic walk: calls `f(key, value)` for every object entry, recursing into everything.
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

/// A node wrapper (`{"node": {"ParamRef": …}}`) that is a parameter, possibly through casts.
fn direct_param_number(wrapper: &Value) -> Option<i32> {
    let inner = wrapper.get("node")?;
    if let Some(p) = inner.get("ParamRef") {
        return p.get("number").and_then(|n| n.as_i64()).map(|n| n as i32);
    }
    if let Some(t) = inner.get("TypeCast") {
        return t.get("arg").and_then(direct_param_number);
    }
    None
}

/// (qualifier, column) for a ColumnRef like `input.q` or bare `q`.
fn column_ref_fields(wrapper: &Value) -> Option<(Option<String>, String)> {
    let cr = wrapper.get("node")?.get("ColumnRef")?;
    let fields = cr.get("fields")?.as_array()?;
    let mut parts: Vec<String> = Vec::new();
    for f in fields {
        let s = f.get("node")?.get("String")?.get("sval")?.as_str()?;
        parts.push(s.to_string());
    }
    match parts.len() {
        1 => Some((None, parts.pop().unwrap())),
        2 => {
            let col = parts.pop().unwrap();
            Some((Some(parts.pop().unwrap()), col))
        }
        _ => None,
    }
}

/// A literal constant as a JSON value, for surfacing coalesce defaults in the schema.
fn const_json(wrapper: &Value) -> Option<Value> {
    let inner = wrapper.get("node")?;
    if let Some(t) = inner.get("TypeCast") {
        return t.get("arg").and_then(const_json);
    }
    let c = inner.get("AConst")?;
    let val = c.get("val")?;
    if let Some(i) = val.get("Ival").and_then(|v| v.get("ival")) {
        return i.as_i64().map(Value::from).or(Some(Value::from(0)));
    }
    if let Some(f) = val.get("Fval").and_then(|v| v.get("fval")).and_then(|v| v.as_str()) {
        return f.parse::<f64>().ok().map(Value::from);
    }
    if let Some(s) = val.get("Sval").and_then(|v| v.get("sval")).and_then(|v| v.as_str()) {
        return Some(Value::from(s));
    }
    if let Some(b) = val.get("Boolval").and_then(|v| v.get("boolval")) {
        return Some(Value::from(b.as_bool().unwrap_or(false)));
    }
    None
}

/// libpg_query error strings carry a location suffix worth keeping, but strip the noisy prefix.
fn clean_parse_error(msg: &str) -> String {
    msg.trim_start_matches("Error { message: \"").trim_end_matches("\" }").to_string()
}

