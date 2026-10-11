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

/// The column a parameter is written to: by name, or by position when an INSERT names no columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filled {
    Name(String),
    Position(usize),
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
    /// Each parameter that is the whole value written to a column, bare or under a cast: an
    /// INSERT's VALUES item, an UPDATE's or an ON CONFLICT's SET. (parameter number, relation, column)
    pub filled: Vec<(i32, String, Filled)>,
    /// Whether RETURNING produces rows (for defaulting `returns`).
    pub has_returning: bool,
    /// Nullability degradation triggers: an outer join anywhere, a repeated table, a set op.
    pub has_outer_join: bool,
    /// Tables on the nullable side of an outer join (the right side of a LEFT JOIN, the left
    /// of a RIGHT JOIN, both of a FULL JOIN): their columns may be null on the result.
    pub nullable_tables: Vec<String>,
    /// The tables a strict top-level WHERE, or a strict ON conjunct of an inner join above the
    /// outer join that nulls them, took off `nullable_tables`, each with every witness that did:
    /// the names of the operators, functions and casts its strictness rests on (none for `IS NOT
    /// NULL`; see `infer::Strict`). A test proves the joined row exists only when those are the
    /// built-in ones, which the parse alone cannot know: a host with the catalog's word puts back
    /// a table no witness holds for.
    pub reductions: Vec<(String, Vec<Vec<String>>)>,
    pub has_repeated_table: bool,
    pub has_set_op: bool,
    /// `GROUP BY ROLLUP`, `CUBE`, or `GROUPING SETS` anywhere: a grouping column is NULL in the
    /// rows that aggregate over it, whatever the catalog says about the column.
    pub has_grouping_sets: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyzeError {
    Parse(String),
    /// The statement nests deeper than libpg_query hands over ([`crate::stack::MAX_DEPTH`]).
    TooDeep,
    NotOneStatement(usize),
    UnsupportedStatement,
}

impl std::fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzeError::Parse(m) => write!(f, "SQL does not parse: {m}"),
            AnalyzeError::TooDeep => write!(
                f,
                "the statement nests deeper than the analysis reads: at most {} levels of its parse tree, which is about {} levels of an expression such as `a + b + …`, a nested CASE or json_build_object, or {} nested subqueries",
                crate::stack::MAX_DEPTH,
                crate::stack::MAX_DEPTH / 2,
                crate::stack::MAX_DEPTH / 6
            ),
            AnalyzeError::NotOneStatement(n) => write!(f, "expected exactly one statement, found {n}"),
            AnalyzeError::UnsupportedStatement => f.write_str("only SELECT, INSERT, UPDATE, DELETE and MERGE statements are typed"),
        }
    }
}

/// Analyze one rewritten statement (placeholders already `$n`).
pub fn analyze(sql: &str) -> Result<Analysis, AnalyzeError> {
    crate::stack::run(|| analyze_here(sql))
}

fn analyze_here(sql: &str) -> Result<Analysis, AnalyzeError> {
    let parsed = crate::stack::parse(sql).map_err(|e| match clean_parse_error(&e.to_string()) {
        m if crate::stack::too_deep(&m) => AnalyzeError::TooDeep,
        m => AnalyzeError::Parse(m),
    })?;
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
    // How many times the statement reads each relation name, anywhere in it.
    let mut occurrences: HashMap<String, usize> = HashMap::new();
    let mut functions: Vec<String> = Vec::new();
    let mut params: HashMap<i32, ParamUse> = HashMap::new();
    let mut filled: Vec<(i32, String, Filled)> = Vec::new();

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
        // A write anywhere makes the statement a mutation, a data-modifying WITH under a SELECT
        // included: `kind` is what a caller routes and wraps in a transaction by. The rows such
        // a statement returns are still the outer SELECT's, which is what `statement` is for.
        "InsertStmt" | "UpdateStmt" | "DeleteStmt" | "MergeStmt" => {
            kind = StatementKind::Mutation;
            if node.get("returning_clause").and_then(|c| c.get("exprs")).and_then(|l| l.as_array()).is_some_and(|l| !l.is_empty()) {
                has_returning = true;
            }
            let Some(relation) = node.get("relation").and_then(|r| r.get("relname")).and_then(|r| r.as_str()) else { return };
            let mut set = |targets: Option<&Value>| {
                for target in targets.and_then(|t| t.as_array()).into_iter().flatten() {
                    let Some(rt) = target.get("node").and_then(|n| n.get("ResTarget")) else { continue };
                    if let (Some(name), Some(n)) = (rt.get("name").and_then(|n| n.as_str()), rt.get("val").and_then(direct_param_number)) {
                        filled.push((n, relation.to_string(), Filled::Name(name.to_string())));
                    }
                }
            };
            match key {
                "UpdateStmt" => set(node.get("target_list")),
                "InsertStmt" => {
                    set(node.get("on_conflict_clause").and_then(|o| o.get("target_list")));
                    let columns: Vec<&str> = node
                        .get("cols")
                        .and_then(|c| c.as_array())
                        .into_iter()
                        .flatten()
                        .filter_map(|c| c.get("node")?.get("ResTarget")?.get("name")?.as_str())
                        .collect();
                    let rows = node.get("select_stmt").and_then(|s| s.get("node")).and_then(|n| n.get("SelectStmt")).and_then(|s| s.get("values_lists")).and_then(|v| v.as_array());
                    for row in rows.into_iter().flatten() {
                        let items = row.get("node").and_then(|n| n.get("List")).and_then(|l| l.get("items")).and_then(|i| i.as_array());
                        for (i, item) in items.into_iter().flatten().enumerate() {
                            let Some(n) = direct_param_number(item) else { continue };
                            let column = if columns.is_empty() { Some(Filled::Position(i)) } else { columns.get(i).map(|c| Filled::Name(c.to_string())) };
                            if let Some(column) = column {
                                filled.push((n, relation.to_string(), column));
                            }
                        }
                    }
                }
                _ => {}
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
                *occurrences.entry(rel.to_string()).or_default() += 1;
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

    // A strict predicate in the top-level WHERE on a column of an outer-joined table matches
    // only rows where that table's row exists, so its columns keep the catalog's nullability:
    // `LEFT JOIN posts p … WHERE p.id IS NOT NULL` (or `p.views > 0`) is an inner join in effect.
    // This is the planner's own outer-join reduction, read off the text. The nullable tables are
    // names, so a relation reduces only where the statement reads it once, and the test's
    // qualifier is that one occurrence's name in the top-level FROM: a test of `a.id` in a
    // self-join, or of a subquery that goes by the table's name, says nothing of the occurrence
    // the join nulls.
    let mut reductions: Vec<(String, Vec<Vec<String>>)> = Vec::new();
    let once = |rel: &str| occurrences.get(rel) == Some(&1);
    if let NodeEnum::SelectStmt(s) = top {
        if let Some(w) = s.where_clause.as_deref() {
            let aliases = alias_map(&s.from_clause);
            let strict = crate::infer::strict_in(w);
            nullable_tables.retain(|rel| {
                if !once(rel) {
                    return true;
                }
                let witnesses: Vec<Vec<String>> = strict
                    .iter()
                    .filter(|st| matches!(st.column.as_slice(), [q, _] if aliases.get(q).is_some_and(|r| r == rel)))
                    .map(|st| st.names.clone())
                    .collect();
                if witnesses.is_empty() {
                    return true;
                }
                reductions.push((rel.clone(), witnesses));
                false
            });
        }
        // A strict ON conjunct of an inner join above the outer join that nulls a relation
        // rejects its null extension too: `posts p LEFT JOIN users u ON … JOIN teams t ON t.id =
        // u.team_id` reads only rows where `u` exists (the planner's `reduce_outer_joins`). The
        // relation must appear once in the statement, in the top-level FROM, so its name is its
        // alone and no outer join above that inner join nulls it again.
        nullable_tables.retain(|rel| {
            if !once(rel) {
                return true;
            }
            match s.from_clause.iter().find_map(|item| reduced_by_joins(item, rel)) {
                Some(witnesses) => {
                    reductions.push((rel.clone(), witnesses));
                    false
                }
                None => true,
            }
        });
    }

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
        filled,
        has_returning,
        has_outer_join,
        nullable_tables,
        reductions,
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
    crate::stack::run(|| params_at_position_here(sql, position))
}

fn params_at_position_here(sql: &str, position: usize) -> Vec<i32> {
    let Some(offset) = sql.char_indices().nth(position.saturating_sub(1)).map(|(b, _)| b as i64) else {
        return Vec::new();
    };
    let Ok(parsed) = crate::stack::parse(sql) else { return Vec::new() };
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
/// The name each FROM item goes by → the relation it is, for the plain tables of a FROM.
fn alias_map(from: &[protobuf::Node]) -> HashMap<String, String> {
    fn walk(node: &protobuf::Node, out: &mut HashMap<String, String>) {
        match node.node.as_ref() {
            Some(NodeEnum::RangeVar(rv)) => {
                let alias = rv.alias.as_ref().map(|a| a.aliasname.clone()).unwrap_or_else(|| rv.relname.clone());
                out.insert(alias, rv.relname.clone());
            }
            Some(NodeEnum::JoinExpr(j)) => {
                if let Some(l) = j.larg.as_deref() {
                    walk(l, out);
                }
                if let Some(r) = j.rarg.as_deref() {
                    walk(r, out);
                }
            }
            _ => {}
        }
    }
    let mut out = HashMap::new();
    for item in from {
        walk(item, &mut out);
    }
    out
}

/// Whether the joins above relation `rel`'s place in a FROM item leave its row existing in every
/// row they return, though an outer join among them nulls it: the witnesses of the inner join's
/// ON conjuncts strict in it that reject its null extension last (`None` when no outer join
/// below nulls it, or when one nulls it again above).
fn reduced_by_joins(item: &protobuf::Node, rel: &str) -> Option<Vec<Vec<String>>> {
    /// The joins from `node` down to the relation, each with whether the relation is on its
    /// right side, and the relation's alias.
    fn path<'a>(node: &'a protobuf::Node, rel: &str, out: &mut Vec<(&'a protobuf::JoinExpr, bool)>) -> Option<String> {
        match node.node.as_ref() {
            Some(NodeEnum::RangeVar(rv)) if rv.relname == rel => Some(rv.alias.as_ref().map_or_else(|| rv.relname.clone(), |a| a.aliasname.clone())),
            Some(NodeEnum::JoinExpr(j)) => {
                for (side, right) in [(j.larg.as_deref(), false), (j.rarg.as_deref(), true)] {
                    out.push((j, right));
                    if let Some(alias) = side.and_then(|s| path(s, rel, out)) {
                        return Some(alias);
                    }
                    out.pop();
                }
                None
            }
            _ => None,
        }
    }
    use protobuf::JoinType;
    let mut joins = Vec::new();
    let alias = path(item, rel, &mut joins)?;
    let mut nulled = false;
    let mut witnesses: Vec<Vec<String>> = Vec::new();
    for (join, right) in joins.iter().rev() {
        let kind = join.jointype;
        if (kind == JoinType::JoinLeft as i32 && *right) || (kind == JoinType::JoinRight as i32 && !*right) || kind == JoinType::JoinFull as i32 {
            nulled = true;
            witnesses.clear();
            continue;
        }
        if nulled && kind == JoinType::JoinInner as i32 && join.using_clause.is_empty() && !join.is_natural {
            let strict: Vec<Vec<String>> = join
                .quals
                .as_deref()
                .map(crate::infer::strict_in)
                .unwrap_or_default()
                .into_iter()
                .filter(|st| matches!(st.column.as_slice(), [q, _] if *q == alias))
                .map(|st| st.names)
                .collect();
            if !strict.is_empty() {
                nulled = false;
                witnesses = strict;
            }
        }
    }
    (!nulled && !witnesses.is_empty()).then_some(witnesses)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_is_query() {
        let a = analyze("SELECT id FROM todos WHERE owner_id = $1").unwrap();
        assert_eq!(a.kind, StatementKind::Query);
        assert_eq!(a.read_set, vec!["todos"]);
        assert!(a.write_set.is_empty());
    }

    #[test]
    fn insert_is_mutation_with_returning() {
        let a = analyze("INSERT INTO todos (title) VALUES ($1) RETURNING id").unwrap();
        assert_eq!(a.kind, StatementKind::Mutation);
        assert!(a.has_returning);
        assert_eq!(a.write_set, vec!["todos"]);
    }

    #[test]
    fn cte_write_makes_select_a_mutation() {
        let a = analyze("WITH ins AS (INSERT INTO audit (x) VALUES ($1) RETURNING id) SELECT id FROM ins").unwrap();
        assert_eq!(a.kind, StatementKind::Mutation);
        assert!(a.write_set.contains(&"audit".to_string()));
    }

    #[test]
    fn locking_clause_detected() {
        let a = analyze("SELECT id FROM todos FOR UPDATE").unwrap();
        assert!(a.has_locking);
    }

    #[test]
    fn coalesce_in_limit_marks_optional_with_default() {
        let a = analyze("SELECT id FROM todos LIMIT coalesce($1::int, 50)").unwrap();
        let p = a.params.get(&1).expect("param 1 analyzed");
        assert!(p.coalesce);
        assert_eq!(p.coalesce_default, Some(serde_json::json!(50)));
    }

    #[test]
    fn coalesce_without_literal_default_is_still_optional() {
        let a = analyze("SELECT id FROM todos WHERE created_at > coalesce($1::timestamptz, now() - interval '1 day')")
            .unwrap();
        let p = a.params.get(&1).unwrap();
        assert!(p.coalesce);
        assert_eq!(p.coalesce_default, None);
    }

    #[test]
    fn a_parameter_written_whole_to_a_column_names_it() {
        use Filled::{Name, Position};
        let a = analyze("INSERT INTO notes (title, body) VALUES ($1, $2::text), ($3, lower($4)) ON CONFLICT (title) DO UPDATE SET body = $5").unwrap();
        let notes = |n: i32, column: &str| (n, "notes".to_string(), Name(column.to_string()));
        assert_eq!(a.filled, vec![notes(5, "body"), notes(1, "title"), notes(2, "body"), notes(3, "title")]);
        let a = analyze("INSERT INTO notes VALUES ($1, $2)").unwrap();
        assert_eq!(a.filled, vec![(1, "notes".into(), Position(0)), (2, "notes".into(), Position(1))]);
        let a = analyze("UPDATE notes SET body = $1, title = upper($2) WHERE id = $3").unwrap();
        assert_eq!(a.filled, vec![(1, "notes".into(), Name("body".into()))]);
        assert!(analyze("SELECT $1::text AS x").unwrap().filled.is_empty());
    }

    #[test]
    fn null_guard_marks_optional() {
        let a = analyze("SELECT id FROM t WHERE ($1::text IS NULL OR title ILIKE '%' || $1 || '%')").unwrap();
        assert!(a.params.get(&1).unwrap().null_guarded);
    }

    #[test]
    fn input_cte_null_guard_traces_through() {
        let a = analyze(
            "WITH input AS (SELECT coalesce($1::int, 50) AS lim, $2::text AS q)
             SELECT id FROM todos, input WHERE (input.q IS NULL OR title ILIKE '%' || input.q || '%')
             LIMIT (SELECT lim FROM input)",
        )
        .unwrap();
        assert_eq!(a.params.get(&1).unwrap().coalesce_default, Some(serde_json::json!(50)));
        assert!(a.params.get(&2).unwrap().null_guarded);
    }

    #[test]
    fn nullable_side_of_outer_joins() {
        let a = analyze("SELECT p.id, u.name FROM posts p LEFT JOIN users u ON u.id = p.author_id").unwrap();
        assert_eq!(a.nullable_tables, vec!["users"]);
        let b = analyze("SELECT * FROM a RIGHT JOIN b ON true").unwrap();
        assert_eq!(b.nullable_tables, vec!["a"]);
        let c = analyze("SELECT * FROM a FULL JOIN b ON true").unwrap();
        assert_eq!(c.nullable_tables, vec!["b", "a"]);
        let d = analyze("SELECT * FROM a JOIN b ON true").unwrap();
        assert!(d.nullable_tables.is_empty());
    }

    #[test]
    fn a_strict_test_in_the_where_or_a_later_inner_join_reduces_an_outer_join() {
        let reduced = |sql: &str| {
            let a = analyze(sql).unwrap();
            (a.nullable_tables, a.reductions.into_iter().map(|(rel, witnesses)| (rel, witnesses.len())).collect::<Vec<_>>())
        };
        // The shapes beyond a bare comparison: a strict function, an OR of strict arms, IN, LIKE.
        for sql in [
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE lower(u.name) = 'x'",
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE u.name LIKE 'a%' OR u.name LIKE 'b%'",
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE u.name IN ('a', 'b')",
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE u.age BETWEEN 1 AND 9",
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE NOT (u.name IS NULL)",
            // A later inner join's ON reads `u`'s column, so the null-extended rows fail it.
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id JOIN teams t ON t.id = u.team_id",
        ] {
            let (nullable, reductions) = reduced(sql);
            assert!(nullable.is_empty(), "{sql}: {nullable:?}");
            assert_eq!(reductions.len(), 1, "{sql}");
        }
        for sql in [
            // One arm is not strict in `u`, an `IS NULL` keeps the null extension, and the
            // later join is itself a LEFT JOIN.
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE u.name = 'a' OR p.id = 1",
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE u.name IS NULL",
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id LEFT JOIN teams t ON t.id = u.team_id",
            // `coalesce` answers for a NULL, and so does `||` over arrays.
            "SELECT p.id FROM posts p LEFT JOIN users u ON u.id = p.author_id WHERE coalesce(u.name, 'x') = 'x'",
            // An outer join above the inner join nulls `u` again.
            "SELECT a.id FROM a LEFT JOIN (posts p LEFT JOIN users u ON u.id = p.author_id JOIN teams t ON t.id = u.team_id) ON p.id = a.post_id",
            // The test reads another occurrence of the table than the one the join nulls, or a
            // subquery that goes by the table's name.
            "SELECT b.name FROM users a LEFT JOIN users b ON b.id = a.boss WHERE a.id = $1",
            "SELECT x.name FROM (SELECT b.name FROM users a LEFT JOIN users b ON b.id = a.boss) x, users u WHERE u.id = $1",
            "SELECT u.name FROM posts p LEFT JOIN users u ON u.id = p.author_id CROSS JOIN (SELECT 1 AS id) AS users WHERE users.id = 1",
            "SELECT u.name FROM posts p LEFT JOIN users u ON u.id = p.author_id JOIN teams t ON t.id = u.team_id, users v WHERE v.id = 1",
        ] {
            let (nullable, _) = reduced(sql);
            assert!(nullable.contains(&"users".to_string()), "{sql}: {nullable:?}");
        }
        // A table the statement reads by its own name.
        let (nullable, _) = reduced("SELECT posts.id FROM posts LEFT JOIN users ON users.id = posts.author_id WHERE users.name = 'x'");
        assert!(nullable.is_empty(), "{nullable:?}");
    }

    #[test]
    fn outer_join_and_repeated_table_flags() {
        let a = analyze("SELECT a.id FROM a LEFT JOIN b ON b.a_id = a.id").unwrap();
        assert!(a.has_outer_join);
        let b = analyze("SELECT x.id FROM t x JOIN t y ON y.parent = x.id").unwrap();
        assert!(b.has_repeated_table);
        let c = analyze("SELECT 1 UNION SELECT 2").unwrap();
        assert!(c.has_set_op);
        assert!(analyze("SELECT owner_id, count(*) FROM todos GROUP BY ROLLUP (owner_id)").unwrap().has_grouping_sets);
        assert!(analyze("SELECT x FROM (SELECT a AS x FROM t GROUP BY CUBE (a)) s").unwrap().has_grouping_sets);
        assert!(!analyze("SELECT owner_id, count(*) FROM todos GROUP BY owner_id").unwrap().has_grouping_sets);
    }

    #[test]
    fn parameters_at_an_error_position() {
        // Postgres points "operator does not exist" at the operator, a datatype mismatch at the
        // expression, and "function does not exist" at the function name.
        let sql = "SELECT id FROM todos WHERE title = $1::int4 AND lower($2::int4) = 'x'";
        let pos = |needle: &str| sql.find(needle).unwrap() + 1;
        assert_eq!(params_at_position(sql, pos("= $1")), vec![1]);
        assert_eq!(params_at_position(sql, pos("$1")), vec![1]);
        assert_eq!(params_at_position(sql, pos("::int4 AND")), vec![1]);
        assert_eq!(params_at_position(sql, pos("lower")), vec![2]);
        assert!(params_at_position(sql, pos("todos")).is_empty());
        let both = "SELECT $1::int4 = $2::text";
        assert_eq!(params_at_position(both, both.find("= $2").unwrap() + 1), vec![1, 2]);
        assert!(params_at_position("SELECT (", 3).is_empty());
    }

    #[test]
    fn two_statements_rejected() {
        assert!(matches!(analyze("SELECT 1; SELECT 2"), Err(AnalyzeError::NotOneStatement(2))));
    }

    #[test]
    fn functions_collected_everywhere() {
        // greatest()/least() parse as MinMaxExpr, not FuncCall; abs() is a real call in LIMIT.
        let a = analyze("SELECT count(*), lower(title) FROM todos LIMIT abs($1::int)").unwrap();
        assert!(a.functions.contains(&"count".to_string()));
        assert!(a.functions.contains(&"lower".to_string()));
        assert!(a.functions.contains(&"abs".to_string()));
    }
}
