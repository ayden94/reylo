//! The soundness half: seed a case's tables with values that test the types, run every statement
//! over them, and report any value that contradicts what the engine said.
//!
//! Everything runs in one transaction that is rolled back, after `SET CONSTRAINTS ALL DEFERRED`,
//! so a deferrable unique constraint lets the duplicates it would refuse at commit stand while the
//! statements run. Each column has a pool of edge values for its type: NULL, case variants of one
//! string, an empty and an unbounded range, `'infinity'`, `NaN`, an array with a NULL element; a
//! literal the column's type refuses is dropped from its pool. Each table gets a base row of the
//! first value of every pool, then one row per value of each pool with the rest of the base row
//! beside it, so every edge value turns up whatever the other columns demand, and a row of NULLs
//! where the table allows it. A row that breaks a unique key gets fresh values in the key's
//! columns and is tried again, so the same values stand twice wherever no immediate key forbids
//! it (a deferrable key, an inheritance parent and its child). A row the table refuses otherwise
//! is skipped (each insert sits in its own savepoint), and the passes repeat so a row that
//! references another table goes in once the referenced row is there. A statement then runs once
//! per assignment of its parameters, each parameter inlined as a literal of the type the engine
//! gave it, and every result row is checked:
//!
//! - a column the engine typed non-null holds no NULL, an array it typed with non-null elements
//!   holds no NULL element, and a JSON field it typed non-null is never null;
//! - a statement proven `one` returns exactly one row, and one proven `maybeOne` at most one.

use serde_json::Value;
use tokio_postgres::Client;

/// One table's columns, in attribute order.
struct Table {
    name: String,
    columns: Vec<Column>,
}

struct Column {
    name: String,
    /// `format_type` of the column's type, the cast a literal is written with.
    ty: String,
    /// The base type's `typcategory` and `typtype`, the element type's for an array.
    category: String,
    kind: String,
    is_array: bool,
    labels: Vec<String>,
    generated: bool,
}

const TABLES_QUERY: &str = "SELECT c.oid::regclass::text, a.attname::text, pg_catalog.format_type(a.atttypid, a.atttypmod), \
    bt.typcategory::text, bt.typtype::text, t.typcategory = 'A', \
    coalesce((SELECT array_agg(e.enumlabel::text ORDER BY e.enumsortorder) FROM pg_enum e WHERE e.enumtypid = bt.oid), '{}'), \
    a.attgenerated <> '' OR a.attidentity = 'a' \
    FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace \
    JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped \
    JOIN pg_type t ON t.oid = a.atttypid \
    JOIN pg_type bt ON bt.oid = CASE WHEN t.typcategory = 'A' THEN t.typelem WHEN t.typtype = 'd' THEN t.typbasetype ELSE t.oid END \
    WHERE c.relkind IN ('r', 'p') AND n.nspname NOT IN ('pg_catalog', 'information_schema') AND n.nspname !~ '^pg_' \
    ORDER BY c.oid, a.attnum";

async fn tables(client: &Client) -> Result<Vec<Table>, String> {
    let rows = client.query(TABLES_QUERY, &[]).await.map_err(|e| e.to_string())?;
    let mut out: Vec<Table> = Vec::new();
    for row in rows {
        let table: String = row.get(0);
        let column = Column {
            name: row.get(1),
            ty: row.get(2),
            category: row.get(3),
            kind: row.get(4),
            is_array: row.get(5),
            labels: row.get(6),
            generated: row.get(7),
        };
        match out.last_mut() {
            Some(t) if t.name == table => t.columns.push(column),
            _ => out.push(Table { name: table, columns: vec![column] }),
        }
    }
    Ok(out)
}

/// The edge values of one column's type, as SQL literals, NULL first.
fn pool(col: &Column) -> Vec<String> {
    let mut scalars: Vec<String> = match (col.kind.as_str(), col.category.as_str()) {
        ("e", _) => col.labels.iter().map(|l| quote(l)).collect(),
        ("r", _) => vec!["'empty'".into(), "'(,5)'".into(), "'[1,)'".into(), "'[1,3)'".into()],
        (_, "N") => vec!["1".into(), "2".into(), "0".into(), "0.1".into(), "0.10000000000000000001".into(), "'NaN'".into()],
        (_, "S") => vec!["'a'".into(), "'A'".into(), "'b'".into(), "''".into(), "'a '".into()],
        (_, "B") => vec!["true".into(), "false".into()],
        (_, "D") => vec!["'2026-01-01'".into(), "'infinity'".into(), "'-infinity'".into(), "'2026-01-01 12:00'".into(), "'12:00'".into()],
        (_, "T") => vec!["'1 day'".into(), "'infinity'".into()],
        (_, "U") if col.ty.contains("json") => vec!["'null'".into(), "'{}'".into(), "'{\"a\": 1}'".into(), "'[]'".into()],
        (_, "U") if col.ty.contains("uuid") => vec!["'00000000-0000-0000-0000-000000000001'".into(), "'00000000-0000-0000-0000-000000000002'".into()],
        (_, "I") => vec!["'10.0.0.1'".into()],
        _ => vec!["'1'".into(), "'a'".into()],
    };
    if col.is_array {
        let element = |v: &str| v.trim_matches('\'').to_string();
        let first = scalars.first().map(|v| element(v)).unwrap_or_else(|| "1".into());
        scalars = vec!["'{}'".into(), format!("'{{{first}}}'"), format!("'{{{first},NULL}}'"), "'{NULL}'".into()];
    }
    let mut out = vec!["NULL".to_string()];
    out.extend(scalars.into_iter().map(|v| format!("{v}::{}", col.ty)));
    out
}

fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

/// Seed every table, inside the transaction the caller opened; the rows inserted per table.
async fn seed(client: &Client, tables: &[Table]) -> Result<Vec<(String, usize)>, String> {
    let mut pools: Vec<Vec<Vec<String>>> = Vec::with_capacity(tables.len());
    for table in tables {
        let mut columns = Vec::new();
        for column in table.columns.iter().filter(|c| !c.generated) {
            let mut accepted = Vec::new();
            for value in pool(column) {
                if value == "NULL" || try_execute(client, &format!("SELECT {value}")).await? {
                    accepted.push(value);
                }
            }
            columns.push(accepted);
        }
        pools.push(columns);
    }
    let mut inserted: Vec<(String, usize)> = tables.iter().map(|t| (t.name.clone(), 0)).collect();
    let mut fresh = 0;
    for _pass in 0..3 {
        for (t, table) in tables.iter().enumerate() {
            let columns: Vec<&Column> = table.columns.iter().filter(|c| !c.generated).collect();
            if columns.is_empty() {
                continue;
            }
            let pools = &pools[t];
            let base: Vec<String> = pools.iter().map(|p| p.iter().find(|v| *v != "NULL").cloned().unwrap_or_else(|| "NULL".into())).collect();
            let mut rows = vec![base.clone(), vec!["NULL".to_string(); columns.len()]];
            for (i, pool) in pools.iter().enumerate() {
                for value in pool {
                    let mut row = base.clone();
                    row[i] = value.clone();
                    rows.push(row);
                }
            }
            for row in rows {
                if insert(client, &table.name, &columns, row, &mut fresh).await? {
                    inserted[t].1 += 1;
                }
            }
        }
    }
    Ok(inserted)
}

/// Insert one row; a row that breaks a unique key is tried again, up to three times, with fresh
/// values in the columns the server names.
async fn insert(client: &Client, table: &str, columns: &[&Column], mut row: Vec<String>, fresh: &mut usize) -> Result<bool, String> {
    let names: Vec<String> = columns.iter().map(|c| format!("\"{}\"", c.name.replace('"', "\"\""))).collect();
    for _ in 0..4 {
        let sql = format!("SAVEPOINT s; INSERT INTO {table} ({}) VALUES ({}); RELEASE SAVEPOINT s", names.join(", "), row.join(", "));
        let error = match client.batch_execute(&sql).await {
            Ok(()) => return Ok(true),
            Err(e) => e,
        };
        client.batch_execute("ROLLBACK TO SAVEPOINT s; RELEASE SAVEPOINT s").await.map_err(|e| e.to_string())?;
        let Some(db) = error.as_db_error() else { return Ok(false) };
        if db.code() != &tokio_postgres::error::SqlState::UNIQUE_VIOLATION {
            return Ok(false);
        }
        // "Key (a, b)=(1, 2) already exists."
        let Some(key) = db.detail().and_then(|d| d.strip_prefix("Key (")).and_then(|d| d.split_once(")=")).map(|(k, _)| k.to_string()) else { return Ok(false) };
        for name in key.split(", ") {
            let Some(i) = columns.iter().position(|c| c.name == name.trim_matches('"')) else { return Ok(false) };
            *fresh += 1;
            row[i] = match fresh_value(columns[i], *fresh) {
                Some(v) => v,
                None => return Ok(false),
            };
        }
    }
    Ok(false)
}

/// A value of the column's type no pool holds, for a key column to take.
fn fresh_value(col: &Column, n: usize) -> Option<String> {
    if col.is_array || col.kind == "e" {
        return None;
    }
    Some(match col.category.as_str() {
        "N" => format!("{}::{}", 1000 + n, col.ty),
        "S" => format!("'k{n}'::{}", col.ty),
        "D" => format!("('2000-01-01'::date + {n})::{}", col.ty),
        "U" if col.ty.contains("uuid") => format!("'00000000-0000-0000-0000-{n:012}'::uuid"),
        _ => return None,
    })
}

/// Whether `sql` runs, in a savepoint that keeps a failure from ending the transaction.
async fn try_execute(client: &Client, sql: &str) -> Result<bool, String> {
    match client.batch_execute(&format!("SAVEPOINT v; {sql}; RELEASE SAVEPOINT v")).await {
        Ok(()) => Ok(true),
        Err(_) => {
            client.batch_execute("ROLLBACK TO SAVEPOINT v; RELEASE SAVEPOINT v").await.map_err(|e| e.to_string())?;
            Ok(false)
        }
    }
}

/// A statement's parameters as literals of their types, several assignments: NULL for a
/// parameter the caller may pass null, and the pool of the parameter's type otherwise.
fn assignments(analysis: &Value, types: &[String]) -> Vec<Vec<String>> {
    let params = analysis["params"].as_array().cloned().unwrap_or_default();
    if params.is_empty() {
        return vec![Vec::new()];
    }
    let pools: Vec<Vec<String>> = params
        .iter()
        .zip(types)
        .map(|(p, ty)| {
            let category = category_of(ty);
            let kind = if p["type"]["kind"] == "enum" { "e" } else if category == "R" { "r" } else { "b" };
            let labels = p["type"]["labels"].as_array().map(|l| l.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
            let col = Column { name: String::new(), ty: ty.clone(), category, kind: kind.into(), is_array: ty.starts_with('_'), labels, generated: false };
            let mut values = pool(&col);
            if !p["nullable"].as_bool().unwrap_or(false) {
                values.retain(|v| v != "NULL");
            }
            // A value no seeded row holds, so an INSERT of a new key runs at all.
            values.extend(fresh_value(&col, 900_000));
            values
        })
        .collect();
    let width = pools.iter().map(Vec::len).max().unwrap_or(1);
    (0..width).map(|r| pools.iter().map(|p| p[r % p.len().max(1)].clone()).collect()).collect()
}

/// A parameter type's category, from its catalog name (`int4`, `_text`): enough to pick a pool.
fn category_of(ty: &str) -> String {
    let base = ty.trim_start_matches('_');
    let c = if ["int2", "int4", "int8", "numeric", "float4", "float8"].contains(&base) {
        "N"
    } else if ["text", "varchar", "bpchar", "name"].contains(&base) {
        "S"
    } else if base == "bool" {
        "B"
    } else if ["date", "timestamp", "timestamptz", "time", "timetz"].contains(&base) {
        "D"
    } else if base == "interval" {
        "T"
    } else if base.contains("range") {
        "R"
    } else {
        "U"
    };
    c.into()
}

/// Run the statements over seeded rows and report every contradiction, as lines.
pub async fn check(client: &Client, statements: &[(String, String, Value)]) -> Result<(Vec<String>, Vec<String>), String> {
    let mut problems = Vec::new();
    let mut notes = Vec::new();
    client.batch_execute("BEGIN; SET CONSTRAINTS ALL DEFERRED").await.map_err(|e| e.to_string())?;
    let outcome = async {
        let tables = tables(client).await?;
        let seeded = seed(client, &tables).await?;
        notes.push(format!("seeded {}", seeded.iter().map(|(t, n)| format!("{t} {n}")).collect::<Vec<_>>().join(", ")));
        for (file, sql, analysis) in statements {
            let mut ran = 0;
            let rewritten = analysis["rewritten"]["sql"].as_str().unwrap_or(sql).to_string();
            let order: Vec<String> = analysis["rewritten"]["order"].as_array().map(|o| o.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
            // Each placeholder `$n` is the n-th name of `order`; the type is that parameter's.
            let params = analysis["params"].as_array().cloned().unwrap_or_default();
            let types: Vec<String> = params.iter().map(|p| p["type_name"].as_str().unwrap_or("text").to_string()).collect();
            for values in assignments(analysis, &types) {
                let mut text = rewritten.clone();
                for (n, name) in order.iter().enumerate().rev() {
                    let index = params.iter().position(|p| p["name"].as_str() == Some(name)).unwrap_or(n);
                    let literal = values.get(index).cloned().unwrap_or_else(|| "NULL".into());
                    let ty = types.get(index).cloned().unwrap_or_else(|| "text".into());
                    text = text.replace(&format!("${}", n + 1), &format!("({literal})::{ty}"));
                }
                let sql_text = format!("SAVEPOINT q; {text}");
                let messages = match client.simple_query(&sql_text).await {
                    Ok(m) => m,
                    Err(_) => {
                        client.batch_execute("ROLLBACK TO SAVEPOINT q").await.map_err(|e| e.to_string())?;
                        continue;
                    }
                };
                client.batch_execute("ROLLBACK TO SAVEPOINT q").await.map_err(|e| e.to_string())?;
                ran += 1;
                let rows: Vec<&tokio_postgres::SimpleQueryRow> = messages.iter().filter_map(|m| match m {
                    tokio_postgres::SimpleQueryMessage::Row(r) => Some(r),
                    _ => None,
                }).collect();
                let cardinality = analysis["cardinality"].as_str().unwrap_or("many");
                if (cardinality == "maybeOne" && rows.len() > 1) || (cardinality == "one" && rows.len() != 1) {
                    problems.push(format!("{file}: proven {cardinality}, returned {} rows with ({})", rows.len(), values.join(", ")));
                }
                let columns = analysis["columns"].as_array().cloned().unwrap_or_default();
                for row in &rows {
                    for (i, col) in columns.iter().enumerate() {
                        let value = row.try_get(i).ok().flatten();
                        let name = col["name"].as_str().unwrap_or("?");
                        if value.is_none() {
                            if !col["nullable"].as_bool().unwrap_or(true) {
                                problems.push(format!("{file}: column {name} typed non-null is NULL with ({})", values.join(", ")));
                            }
                            continue;
                        }
                        if let Some(why) = contradiction(&col["type"], value.unwrap_or_default()) {
                            problems.push(format!("{file}: column {name} {why} with ({})", values.join(", ")));
                        }
                    }
                }
            }
            // A statement that never ran checked nothing, which a reader must see.
            if ran == 0 {
                notes.push(format!("{file} failed under every assignment, so nothing about it was checked"));
            }
        }
        Ok::<(), String>(())
    }
    .await;
    client.batch_execute("ROLLBACK").await.map_err(|e| e.to_string())?;
    outcome?;
    problems.sort();
    problems.dedup();
    Ok((problems, notes))
}

/// Why a non-NULL value contradicts its type, if it does: a NULL element in an array typed without
/// one, or a null where a JSON object or list typed a value.
fn contradiction(ty: &Value, text: &str) -> Option<String> {
    match ty["kind"].as_str() {
        // A union of literals, from an enum or a CHECK list: the value is one of them.
        Some("enum") => {
            let labels: Vec<&str> = ty["labels"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
            (!labels.is_empty() && !labels.contains(&text)).then(|| format!("typed as one of {labels:?} holds {text:?}"))
        }
        Some("array") if !ty["nullable_items"].as_bool().unwrap_or(true) => array_has_null(text).then(|| "typed with non-null elements holds a NULL element".to_string()),
        Some("object") | Some("list") => {
            let json: Value = serde_json::from_str(text).ok()?;
            json_contradiction(ty, &json, "")
        }
        _ => None,
    }
}

fn array_has_null(text: &str) -> bool {
    // An unquoted NULL between delimiters is a NULL element; a quoted "NULL" is a string.
    let mut in_quotes = false;
    let mut token = String::new();
    let mut found = false;
    let mut check = |token: &mut String| {
        if token.trim().eq_ignore_ascii_case("null") {
            found = true;
        }
        token.clear();
    };
    let mut escaped = false;
    for c in text.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' => escaped = true,
            '"' => in_quotes = !in_quotes,
            ',' | '{' | '}' if !in_quotes => check(&mut token),
            _ if in_quotes => {}
            _ => token.push(c),
        }
    }
    check(&mut token);
    found
}

fn json_contradiction(ty: &Value, json: &Value, path: &str) -> Option<String> {
    match ty["kind"].as_str() {
        Some("object") => {
            let obj = json.as_object()?;
            for f in ty["fields"].as_array()? {
                let name = f["name"].as_str()?;
                let v = obj.get(name).unwrap_or(&Value::Null);
                if v.is_null() {
                    if !f["nullable"].as_bool().unwrap_or(true) {
                        return Some(format!("has a null at {path}.{name}, typed non-null"));
                    }
                    continue;
                }
                if let Some(why) = json_contradiction(&f["type"], v, &format!("{path}.{name}")) {
                    return Some(why);
                }
            }
            None
        }
        Some("list") => {
            for (i, item) in json.as_array()?.iter().enumerate() {
                if item.is_null() {
                    if !ty["nullable_items"].as_bool().unwrap_or(true) {
                        return Some(format!("has a null item at {path}[{i}], typed non-null"));
                    }
                    continue;
                }
                if let Some(why) = json_contradiction(&ty["of"], item, &format!("{path}[{i}]")) {
                    return Some(why);
                }
            }
            None
        }
        Some("array") if !ty["nullable_items"].as_bool().unwrap_or(true) => json.as_array()?.iter().any(Value::is_null).then(|| format!("has a null element at {path}, typed non-null")),
        _ => None,
    }
}

// ---------------------------------------------------------------------------------------------
// SQLite
// ---------------------------------------------------------------------------------------------

/// The SQLite half of the same check, in one transaction that is rolled back. Foreign keys are
/// off, as they are by default, so a row may reference nothing and an outer join extends it with
/// NULLs. A pool per declared type, by SQLite's affinity rules; a statement runs once per
/// assignment of its parameters, each bound as a value.
pub fn check_sqlite(db: &rusqlite::Connection, statements: &[(String, String, Value)]) -> Result<(Vec<String>, Vec<String>), String> {
    let mut problems = Vec::new();
    let mut notes = Vec::new();
    db.execute_batch("BEGIN").map_err(|e| e.to_string())?;
    let outcome = (|| -> Result<(), String> {
        let seeded = seed_sqlite(db)?;
        notes.push(format!("seeded {}", seeded.iter().map(|(t, n)| format!("{t} {n}")).collect::<Vec<_>>().join(", ")));
        for (file, sql, analysis) in statements {
            let text = analysis["rewritten"]["sql"].as_str().unwrap_or(sql).to_string();
            let params = analysis["params"].as_array().cloned().unwrap_or_default();
            let order: Vec<String> = analysis["rewritten"]["order"].as_array().map(|o| o.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
            let pools: Vec<Vec<rusqlite::types::Value>> = params
                .iter()
                .map(|p| {
                    let mut values = sqlite_pool(p["type_name"].as_str().unwrap_or(""));
                    if !p["nullable"].as_bool().unwrap_or(false) {
                        values.retain(|v| *v != rusqlite::types::Value::Null);
                    }
                    values
                })
                .collect();
            let width = pools.iter().map(Vec::len).max().unwrap_or(1).max(1);
            let mut ran = 0;
            for r in 0..width {
                let values: Vec<rusqlite::types::Value> = pools.iter().map(|p| p.get(r % p.len().max(1)).cloned().unwrap_or(rusqlite::types::Value::Null)).collect();
                // Each `?` is the n-th name of `order`, and the value is that parameter's.
                let bound: Vec<rusqlite::types::Value> = order
                    .iter()
                    .map(|name| params.iter().position(|p| p["name"].as_str() == Some(name)).and_then(|i| values.get(i).cloned()).unwrap_or(rusqlite::types::Value::Null))
                    .collect();
                let shown = bound.iter().map(|v| format!("{v:?}")).collect::<Vec<_>>().join(", ");
                db.execute_batch("SAVEPOINT q").map_err(|e| e.to_string())?;
                let rows = run_sqlite_statement(db, &text, &bound);
                db.execute_batch("ROLLBACK TO q; RELEASE q").map_err(|e| e.to_string())?;
                let Ok(rows) = rows else { continue };
                ran += 1;
                let cardinality = analysis["cardinality"].as_str().unwrap_or("many");
                if (cardinality == "maybeOne" && rows.len() > 1) || (cardinality == "one" && rows.len() != 1) {
                    problems.push(format!("{file}: proven {cardinality}, returned {} rows with ({shown})", rows.len()));
                }
                let columns = analysis["columns"].as_array().cloned().unwrap_or_default();
                for row in &rows {
                    for (i, col) in columns.iter().enumerate() {
                        let name = col["name"].as_str().unwrap_or("?");
                        match row.get(i) {
                            Some(rusqlite::types::Value::Null) if !col["nullable"].as_bool().unwrap_or(true) => {
                                problems.push(format!("{file}: column {name} typed non-null is NULL with ({shown})"));
                            }
                            Some(rusqlite::types::Value::Text(text)) => {
                                if let Some(why) = contradiction(&col["type"], text) {
                                    problems.push(format!("{file}: column {name} {why} with ({shown})"));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            if ran == 0 {
                notes.push(format!("{file} failed under every assignment, so nothing about it was checked"));
            }
        }
        Ok(())
    })();
    db.execute_batch("ROLLBACK").map_err(|e| e.to_string())?;
    outcome?;
    problems.sort();
    problems.dedup();
    Ok((problems, notes))
}

fn run_sqlite_statement(db: &rusqlite::Connection, sql: &str, values: &[rusqlite::types::Value]) -> Result<Vec<Vec<rusqlite::types::Value>>, rusqlite::Error> {
    let mut stmt = db.prepare(sql)?;
    let width = stmt.column_count();
    let rows = stmt.query_map(rusqlite::params_from_iter(values.iter()), |row| (0..width).map(|i| row.get::<_, rusqlite::types::Value>(i)).collect())?;
    rows.collect()
}

/// The edge values of a declared SQLite type, NULL first, by the affinity SQLite gives the name.
fn sqlite_pool(declared: &str) -> Vec<rusqlite::types::Value> {
    use rusqlite::types::Value::{Blob, Integer, Null, Real, Text};
    let t = declared.to_ascii_lowercase();
    let text = || vec![Text("a".into()), Text("A".into()), Text(String::new()), Text("a ".into())];
    let mut out = vec![Null];
    if t.contains("int") {
        out.extend([Integer(1), Integer(2), Integer(0), Integer(-1), Integer(i64::MAX)]);
    } else if t.contains("char") || t.contains("clob") || t.contains("text") {
        out.extend(text());
    } else if t.contains("blob") {
        out.extend([Blob(vec![0]), Blob(Vec::new())]);
    } else if t.contains("real") || t.contains("floa") || t.contains("doub") {
        out.extend([Real(1.5), Real(0.0), Real(-0.0), Real(1e308)]);
    } else if t.is_empty() {
        out.extend([Integer(1), Text("a".into())]);
    } else {
        out.extend([Integer(1), Real(1.5), Text("1".into()), Text("x".into())]);
    }
    out
}

/// Seed every table in the database with a base row and one row per pool value of each column;
/// a row that breaks a unique key gets fresh values in the key's columns and is tried again.
fn seed_sqlite(db: &rusqlite::Connection) -> Result<Vec<(String, usize)>, String> {
    let tables: Vec<String> = {
        let mut stmt = db.prepare("select name from sqlite_schema where type = 'table' and name not like 'sqlite_%' order by rowid").map_err(|e| e.to_string())?;
        let names = stmt.query_map([], |r| r.get::<_, String>(0)).map_err(|e| e.to_string())?;
        names.collect::<Result<_, _>>().map_err(|e| e.to_string())?
    };
    let mut out = Vec::new();
    let mut fresh = 0i64;
    for table in &tables {
        let columns: Vec<(String, String, bool)> = {
            let mut stmt = db.prepare(&format!("select name, type, hidden from pragma_table_xinfo('{}')", table.replace('\'', "''"))).map_err(|e| e.to_string())?;
            let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, i64>(2)? != 0))).map_err(|e| e.to_string())?;
            rows.collect::<Result<_, _>>().map_err(|e| e.to_string())?
        };
        let columns: Vec<(String, String)> = columns.into_iter().filter(|(_, _, hidden)| !hidden).map(|(n, t, _)| (n, t)).collect();
        if columns.is_empty() {
            continue;
        }
        let pools: Vec<Vec<rusqlite::types::Value>> = columns.iter().map(|(_, t)| sqlite_pool(t)).collect();
        let base: Vec<rusqlite::types::Value> = pools.iter().map(|p| p.get(1).cloned().unwrap_or(rusqlite::types::Value::Null)).collect();
        let mut rows = vec![base.clone(), vec![rusqlite::types::Value::Null; columns.len()]];
        for (i, pool) in pools.iter().enumerate() {
            for v in pool {
                let mut row = base.clone();
                row[i] = v.clone();
                rows.push(row);
            }
        }
        let names: Vec<String> = columns.iter().map(|(n, _)| format!("\"{}\"", n.replace('"', "\"\""))).collect();
        let marks: Vec<&str> = columns.iter().map(|_| "?").collect();
        let sql = format!("insert into \"{}\" ({}) values ({})", table.replace('"', "\"\""), names.join(", "), marks.join(", "));
        let mut inserted = 0;
        for mut row in rows {
            for _ in 0..4 {
                match db.execute(&sql, rusqlite::params_from_iter(row.iter())) {
                    Ok(_) => {
                        inserted += 1;
                        break;
                    }
                    Err(rusqlite::Error::SqliteFailure(e, Some(message))) if e.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE || e.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY => {
                        // "UNIQUE constraint failed: t.a, t.b"
                        let failed: Vec<String> = message.split(": ").nth(1).unwrap_or("").split(", ").filter_map(|c| c.split('.').nth(1).map(String::from)).collect();
                        let mut changed = false;
                        for column in failed {
                            if let Some(i) = columns.iter().position(|(n, _)| *n == column) {
                                fresh += 1;
                                row[i] = if columns[i].1.to_ascii_lowercase().contains("int") { rusqlite::types::Value::Integer(1000 + fresh) } else { rusqlite::types::Value::Text(format!("k{fresh}")) };
                                changed = true;
                            }
                        }
                        if !changed {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
        out.push((table.clone(), inserted));
    }
    Ok(out)
}
