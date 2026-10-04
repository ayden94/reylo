//! The engine against live databases.
//!
//! A case is a directory: `setup.sql` builds a schema, every other `.sql` file is one statement,
//! and `expected.txt` holds one line per statement in the form this tool prints. A directory with
//! no `expected.txt`, such as the parity check's statements, is checked for soundness alone. A Postgres case
//! runs in a database of its own, created on the server `--url` names and dropped afterwards, so
//! the server must be a throwaway one: cases create collations, operators and roles. A case whose
//! `dialect` file says `sqlite` runs in a fresh SQLite file instead. A case with a `min_version`
//! file (a `server_version_num`, such as `180000`) is skipped on an older server.
//!
//!   cargo run -p porg-oracle -- --url postgres://postgres@localhost:55432/postgres oracle/cases
//!   cargo run -p porg-oracle -- --url … --bless oracle/cases/not_null   # rewrite expected.txt
//!   cargo run -p porg-oracle -- --url … --analyses old.json oracle/cases  # check other answers
//!   cargo run -p porg-oracle -- --url … --dump new.json oracle/cases      # write the answers
//!
//! `--analyses` names a JSON object of analyses keyed `<case>/<file>`, which the seeding checks
//! in place of the engine's own: how a check that passes shows it can fail, run on the answers of
//! an engine that had a defect.
//!
//! Each statement goes through the engine's replay exactly as a host runs it: the describe comes
//! from the extended protocol and every catalog query runs on the case's database. A catalog
//! query that fails is a failure of the case, never an empty answer.

mod seed;

use serde_json::{json, Map, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr;
use tokio_postgres::types::{ToSql, Type};
use tokio_postgres::{Client, Config, NoTls};

#[tokio::main]
async fn main() -> ExitCode {
    let mut url = std::env::var("ORACLE_URL").ok();
    let mut bless = false;
    let mut keep = false;
    let mut given = Map::new();
    let mut dump: Option<PathBuf> = None;
    let mut paths = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--url" => url = args.next(),
            "--bless" => bless = true,
            "--keep" => keep = true,
            "--dump" => dump = args.next().map(PathBuf::from),
            "--analyses" => {
                let path = args.next().unwrap_or_default();
                match fs::read_to_string(&path).ok().and_then(|text| serde_json::from_str::<Value>(&text).ok()) {
                    Some(Value::Object(map)) => given = map,
                    _ => {
                        eprintln!("--analyses {path}: not a JSON object of analyses");
                        return ExitCode::from(2);
                    }
                }
            }
            _ => paths.push(PathBuf::from(arg)),
        }
    }
    let cases = cases(&paths);
    if cases.is_empty() {
        eprintln!("no cases: name a case directory, or a directory of them");
        return ExitCode::from(2);
    }
    let mut failed = 0;
    let mut dumped = Map::new();
    let mut admin: Option<(Client, Config, i32)> = None;
    for case in &cases {
        let dialect = fs::read_to_string(case.join("dialect")).map(|s| s.trim().to_string()).unwrap_or_else(|_| "postgres".into());
        let outcome = if dialect == "sqlite" {
            run_sqlite(case)
        } else {
            if admin.is_none() {
                let Some(url) = url.as_deref() else {
                    eprintln!("a Postgres case needs --url (or ORACLE_URL): a throwaway server, never a shared one");
                    return ExitCode::from(2);
                };
                match connect_admin(url).await {
                    Ok(a) => admin = Some(a),
                    Err(e) => {
                        eprintln!("cannot connect to {url}: {e}");
                        return ExitCode::from(2);
                    }
                }
            }
            let (client, config, version) = admin.as_ref().expect("connected above");
            let min = fs::read_to_string(case.join("min_version")).ok().and_then(|s| s.trim().parse::<i32>().ok()).unwrap_or(0);
            if *version < min {
                println!("{}: skipped (server {version} < {min})", case.display());
                continue;
            }
            run_postgres(case, client, config, keep, &given, &mut dumped).await
        };
        match outcome {
            Ok(lines) => {
                if !check(case, &lines, bless) {
                    failed += 1;
                }
            }
            Err(e) => {
                println!("{}: FAILED: {e}", case.display());
                failed += 1;
            }
        }
    }
    if let Some(path) = dump {
        fs::write(&path, serde_json::to_string_pretty(&Value::Object(dumped)).unwrap_or_default()).expect("the --dump file is writable");
    }
    println!("{} case(s), {failed} failed", cases.len());
    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// The case directories under `paths`: a directory with a `setup.sql` is a case, and any other
/// directory is searched one level down.
fn cases(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for path in paths {
        if path.join("setup.sql").is_file() {
            out.push(path.clone());
        } else if let Ok(entries) = fs::read_dir(path) {
            let mut dirs: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.join("setup.sql").is_file()).collect();
            dirs.sort();
            out.extend(dirs);
        }
    }
    out
}

/// The statements of a case, by file name.
fn statements(case: &Path) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = fs::read_dir(case)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "sql") && p.file_name().is_some_and(|n| n != "setup.sql"))
                .filter_map(|p| Some((p.file_name()?.to_string_lossy().to_string(), fs::read_to_string(&p).ok()?)))
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

/// Compare a case's lines with its `expected.txt`, or write them there under `--bless`.
fn check(case: &Path, lines: &[String], bless: bool) -> bool {
    let path = case.join("expected.txt");
    let actual = lines.join("\n") + "\n";
    if bless {
        fs::write(&path, &actual).expect("expected.txt is writable");
        println!("{}: blessed {} statement(s)", case.display(), lines.len());
        return true;
    }
    let Ok(expected) = fs::read_to_string(&path) else {
        println!("{}: sound ({} statement(s), no expected.txt)", case.display(), lines.len());
        return true;
    };
    if expected == actual {
        println!("{}: ok ({} statement(s))", case.display(), lines.len());
        return true;
    }
    println!("{}: MISMATCH", case.display());
    let want: Vec<&str> = expected.lines().collect();
    for line in lines {
        if !want.contains(&line.as_str()) {
            println!("  + {line}");
        }
    }
    for line in &want {
        if !lines.iter().any(|l| l == line) {
            println!("  - {line}");
        }
    }
    false
}

// ---------------------------------------------------------------------------------------------
// Postgres
// ---------------------------------------------------------------------------------------------

async fn connect(config: &Config) -> Result<Client, String> {
    let (client, connection) = config.connect(NoTls).await.map_err(|e| e.to_string())?;
    tokio::spawn(async move {
        let _ = connection.await;
    });
    Ok(client)
}

async fn connect_admin(url: &str) -> Result<(Client, Config, i32), String> {
    let config = Config::from_str(url).map_err(|e| e.to_string())?;
    if config.get_ports().contains(&5432) {
        return Err("port 5432 is the shared server; the oracle runs only on a throwaway cluster".into());
    }
    let client = connect(&config).await?;
    let version: String = client.query_one("SHOW server_version_num", &[]).await.map_err(|e| e.to_string())?.get(0);
    Ok((client, config, version.parse().unwrap_or(0)))
}

async fn run_postgres(case: &Path, admin: &Client, admin_config: &Config, keep: bool, given: &Map<String, Value>, dumped: &mut Map<String, Value>) -> Result<Vec<String>, String> {
    let name: String = case.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let db = format!("porg_oracle_{}", name.chars().map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' }).collect::<String>());
    // Two messages: in one, CREATE DATABASE would run inside the batch's implicit transaction.
    admin.batch_execute(&format!("DROP DATABASE IF EXISTS {db} WITH (FORCE)")).await.map_err(|e| format!("drop {db}: {}", pg_message(&e)))?;
    admin.batch_execute(&format!("CREATE DATABASE {db}")).await.map_err(|e| format!("create {db}: {}", pg_message(&e)))?;
    let mut config = admin_config.clone();
    config.dbname(&db);
    let client = connect(&config).await?;
    let setup = fs::read_to_string(case.join("setup.sql")).map_err(|e| e.to_string())?;
    client.batch_execute(&setup).await.map_err(|e| format!("setup.sql: {}", pg_message(&e)))?;
    let mut lines = Vec::new();
    let mut analyzed = Vec::new();
    for (file, sql) in statements(case) {
        let analysis = match given.get(&format!("{name}/{file}")) {
            Some(answer) => answer.clone(),
            None => analyze_postgres(&client, &sql).await.map_err(|e| format!("{file}: {e}"))?,
        };
        lines.push(format!("{file}: {}", line(&analysis)));
        dumped.insert(format!("{name}/{file}"), analysis.clone());
        analyzed.push((file, sql, analysis));
    }
    // A value that contradicts a type is a failure whatever expected.txt says.
    let (problems, notes) = seed::check(&client, &analyzed).await.map_err(|e| format!("seeding: {e}"))?;
    for note in &notes {
        println!("{}: note: {note}", case.display());
    }
    drop(client);
    if !problems.is_empty() {
        if !keep {
            let _ = admin.batch_execute(&format!("DROP DATABASE IF EXISTS {db} WITH (FORCE)")).await;
        }
        return Err(format!("{} value(s) contradict the types:\n    {}", problems.len(), problems.join("\n    ")));
    }
    if !keep {
        let _ = admin.batch_execute(&format!("DROP DATABASE IF EXISTS {db} WITH (FORCE)")).await;
    }
    Ok(lines)
}

/// The engine's replay with every need answered on `client`.
async fn analyze_postgres(client: &Client, sql: &str) -> Result<Value, String> {
    let mut answers = Map::new();
    for _ in 0..64 {
        let request = json!({ "op": "analyze", "sql": sql, "dialect": "postgres", "answers": Value::Object(answers.clone()) });
        let response: Value = serde_json::from_str(&porg_engine::call(&request.to_string())).map_err(|e| e.to_string())?;
        match response["status"].as_str() {
            Some("analysis") => return Ok(response["analysis"].clone()),
            Some("need") => {
                for need in response["needs"].as_array().cloned().unwrap_or_default() {
                    let id = need["id"].as_str().unwrap_or_default().to_string();
                    let answer = fulfil_postgres(client, &need).await?;
                    answers.insert(id, answer);
                }
            }
            _ => return Err(format!("the engine answered {response}")),
        }
    }
    Err("the replay did not converge".into())
}

async fn fulfil_postgres(client: &Client, need: &Value) -> Result<Value, String> {
    let sql = need["sql"].as_str().unwrap_or_default();
    match need["kind"].as_str() {
        Some("describe") => Ok(match client.prepare(sql).await {
            Ok(stmt) => {
                let params: Vec<u32> = stmt.params().iter().map(Type::oid).collect();
                let columns: Vec<Value> = stmt
                    .columns()
                    .iter()
                    .map(|c| json!({ "name": c.name(), "oid": c.type_().oid(), "table": c.table_oid().unwrap_or(0), "attnum": c.column_id().map_or(0, i32::from) }))
                    .collect();
                json!({ "describe": { "params": params, "columns": columns } })
            }
            Err(e) => json!({ "error": pg_error(&e) }),
        }),
        Some("query") => {
            let params = need["params"].as_array().cloned().unwrap_or_default();
            let stmt = client.prepare(sql).await.map_err(|e| format!("catalog query failed to prepare: {}\n    {sql}", pg_message(&e)))?;
            let mut values: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
            for (ty, value) in stmt.params().iter().zip(&params) {
                values.push(bind(ty, value).ok_or_else(|| format!("cannot bind {value} as {ty} for the catalog query\n    {sql}"))?);
            }
            let refs: Vec<&(dyn ToSql + Sync)> = values.iter().map(|v| v.as_ref() as &(dyn ToSql + Sync)).collect();
            let rows = client.query(&stmt, &refs).await.map_err(|e| format!("catalog query failed: {}\n    {sql}", pg_message(&e)))?;
            Ok(json!({ "rows": rows.iter().map(|row| (0..row.len()).map(|i| cell(row, i)).collect::<Vec<Value>>()).collect::<Vec<_>>() }))
        }
        other => Err(format!("unknown need {other:?}")),
    }
}

/// One parameter of a catalog query, by the type the server inferred from the cast in its text.
fn bind(ty: &Type, value: &Value) -> Option<Box<dyn ToSql + Sync + Send>> {
    let items = || value.as_array().cloned().unwrap_or_default();
    Some(match *ty {
        Type::OID_ARRAY => Box::new(items().iter().map(|v| v.as_u64().map(|n| n as u32)).collect::<Option<Vec<u32>>>()?),
        Type::INT2_ARRAY => Box::new(items().iter().map(|v| v.as_i64().map(|n| n as i16)).collect::<Option<Vec<i16>>>()?),
        Type::INT4_ARRAY => Box::new(items().iter().map(|v| v.as_i64().map(|n| n as i32)).collect::<Option<Vec<i32>>>()?),
        Type::INT8_ARRAY => Box::new(items().iter().map(|v| v.as_i64()).collect::<Option<Vec<i64>>>()?),
        Type::TEXT_ARRAY | Type::NAME_ARRAY | Type::VARCHAR_ARRAY => Box::new(items().iter().map(|v| v.as_str().map(String::from)).collect::<Option<Vec<String>>>()?),
        Type::TEXT | Type::VARCHAR | Type::NAME => Box::new(value.as_str().map(String::from).unwrap_or_else(|| value.to_string())),
        Type::OID => Box::new(value.as_u64()? as u32),
        Type::INT2 => Box::new(value.as_i64()? as i16),
        Type::INT4 => Box::new(value.as_i64()? as i32),
        Type::INT8 => Box::new(value.as_i64()?),
        Type::BOOL => Box::new(value.as_bool()?),
        _ => return None,
    })
}

/// One value of a catalog row as JSON, by its column's type.
fn cell(row: &tokio_postgres::Row, i: usize) -> Value {
    fn array<T: Into<Value>>(items: Option<Vec<Option<T>>>) -> Option<Value> {
        items.map(|items| Value::Array(items.into_iter().map(|v| v.map_or(Value::Null, Into::into)).collect()))
    }
    let value = match *row.columns()[i].type_() {
        Type::BOOL => row.get::<_, Option<bool>>(i).map(Value::from),
        Type::INT2 => row.get::<_, Option<i16>>(i).map(Value::from),
        Type::INT4 => row.get::<_, Option<i32>>(i).map(Value::from),
        Type::INT8 => row.get::<_, Option<i64>>(i).map(Value::from),
        Type::OID => row.get::<_, Option<u32>>(i).map(Value::from),
        Type::TEXT | Type::VARCHAR | Type::NAME | Type::BPCHAR | Type::CHAR => row.get::<_, Option<String>>(i).map(Value::from),
        Type::TEXT_ARRAY | Type::VARCHAR_ARRAY | Type::NAME_ARRAY => array(row.get::<_, Option<Vec<Option<String>>>>(i)),
        Type::INT2_ARRAY => array(row.get::<_, Option<Vec<Option<i16>>>>(i)),
        Type::INT4_ARRAY => array(row.get::<_, Option<Vec<Option<i32>>>>(i)),
        Type::INT8_ARRAY => array(row.get::<_, Option<Vec<Option<i64>>>>(i)),
        Type::OID_ARRAY => array(row.get::<_, Option<Vec<Option<u32>>>>(i)),
        Type::BOOL_ARRAY => array(row.get::<_, Option<Vec<Option<bool>>>>(i)),
        ref other => panic!("a catalog query answered a column of type {other}, which no host binds"),
    };
    value.unwrap_or(Value::Null)
}

fn pg_message(e: &tokio_postgres::Error) -> String {
    e.as_db_error().map(|d| format!("{} ({})", d.message(), d.code().code())).unwrap_or_else(|| e.to_string())
}

fn pg_error(e: &tokio_postgres::Error) -> Value {
    match e.as_db_error() {
        Some(d) => {
            let position = match d.position() {
                Some(tokio_postgres::error::ErrorPosition::Original(p)) => Some(*p),
                _ => None,
            };
            json!({ "message": d.message(), "code": d.code().code(), "position": position })
        }
        None => json!({ "message": e.to_string() }),
    }
}

// ---------------------------------------------------------------------------------------------
// SQLite
// ---------------------------------------------------------------------------------------------

fn run_sqlite(case: &Path) -> Result<Vec<String>, String> {
    let name: String = case.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let file = std::env::temp_dir().join(format!("porg-oracle-{name}.db"));
    let _ = fs::remove_file(&file);
    let db = rusqlite::Connection::open(&file).map_err(|e| e.to_string())?;
    let setup = fs::read_to_string(case.join("setup.sql")).map_err(|e| e.to_string())?;
    db.execute_batch(&setup).map_err(|e| format!("setup.sql: {e}"))?;
    let mut lines = Vec::new();
    let mut analyzed = Vec::new();
    for (file_name, sql) in statements(case) {
        let analysis = analyze_sqlite(&db, &sql).map_err(|e| format!("{file_name}: {e}"))?;
        lines.push(format!("{file_name}: {}", line(&analysis)));
        analyzed.push((file_name, sql, analysis));
    }
    // A value that contradicts a type is a failure whatever expected.txt says.
    let (problems, notes) = seed::check_sqlite(&db, &analyzed).map_err(|e| format!("seeding: {e}"))?;
    for note in &notes {
        println!("{}: note: {note}", case.display());
    }
    drop(db);
    let _ = fs::remove_file(&file);
    if !problems.is_empty() {
        return Err(format!("{} value(s) contradict the types:\n    {}", problems.len(), problems.join("\n    ")));
    }
    Ok(lines)
}

fn analyze_sqlite(db: &rusqlite::Connection, sql: &str) -> Result<Value, String> {
    let mut answers = Map::new();
    for _ in 0..16 {
        let request = json!({ "op": "analyze", "sql": sql, "dialect": "sqlite", "answers": Value::Object(answers.clone()) });
        let response: Value = serde_json::from_str(&porg_engine::call(&request.to_string())).map_err(|e| e.to_string())?;
        match response["status"].as_str() {
            Some("analysis") => return Ok(response["analysis"].clone()),
            Some("need") => {
                for need in response["needs"].as_array().cloned().unwrap_or_default() {
                    let id = need["id"].as_str().unwrap_or_default().to_string();
                    if need["kind"] != "query" {
                        return Err(format!("the SQLite path asked for {need}"));
                    }
                    let query = need["sql"].as_str().unwrap_or_default();
                    let mut stmt = db.prepare(query).map_err(|e| format!("catalog query: {e}"))?;
                    let width = stmt.column_count();
                    let rows: Vec<Value> = stmt
                        .query_map([], |row| {
                            (0..width)
                                .map(|i| {
                                    Ok(match row.get_ref(i)? {
                                        rusqlite::types::ValueRef::Null => Value::Null,
                                        rusqlite::types::ValueRef::Integer(n) => json!(n),
                                        rusqlite::types::ValueRef::Real(f) => json!(f),
                                        rusqlite::types::ValueRef::Text(t) => json!(String::from_utf8_lossy(t)),
                                        rusqlite::types::ValueRef::Blob(_) => Value::Null,
                                    })
                                })
                                .collect::<Result<Vec<Value>, rusqlite::Error>>()
                                .map(Value::Array)
                        })
                        .map_err(|e| e.to_string())?
                        .collect::<Result<Vec<Value>, rusqlite::Error>>()
                        .map_err(|e| e.to_string())?;
                    answers.insert(id, json!({ "rows": rows }));
                }
            }
            _ => return Err(format!("the engine answered {response}")),
        }
    }
    Err("the replay did not converge".into())
}

// ---------------------------------------------------------------------------------------------
// The printed form
// ---------------------------------------------------------------------------------------------

/// `maybeOne (id: int4) -> { title: text, author: text? }`, then any diagnostics.
fn line(analysis: &Value) -> String {
    let field = |v: &Value| format!("{}: {}{}", v["name"].as_str().unwrap_or("?"), render(&v["type"]), if v["nullable"].as_bool().unwrap_or(false) { "?" } else { "" });
    let params: Vec<String> = analysis["params"].as_array().map(|a| a.iter().map(field).collect()).unwrap_or_default();
    let columns: Vec<String> = analysis["columns"].as_array().map(|a| a.iter().map(field).collect()).unwrap_or_default();
    let mut out = format!("{} ({}) -> {{ {} }}", analysis["cardinality"].as_str().unwrap_or("?"), params.join(", "), columns.join(", "));
    for d in analysis["diagnostics"].as_array().into_iter().flatten() {
        out.push_str(&format!(" !! porg{}: {}", d["code"], d["message"].as_str().unwrap_or("")));
    }
    out
}

/// A type in one short line: `int4`, `'a' | 'b'`, `text[]`, `{ id: int4 }`, `[{ … }?]`. An array
/// the statement builds says what it knows of its elements: `(text?)[]` when one may be NULL,
/// `(text!)[]` when none can be.
fn render(ty: &Value) -> String {
    let fields = |v: &Value| -> String {
        let parts: Vec<String> = v.as_array().into_iter().flatten().map(|f| format!("{}: {}{}", f["name"].as_str().unwrap_or("?"), render(&f["type"]), if f["nullable"].as_bool().unwrap_or(false) { "?" } else { "" })).collect();
        format!("{{ {} }}", parts.join(", "))
    };
    match ty["kind"].as_str() {
        Some("scalar") => ty["name"].as_str().unwrap_or("?").to_string(),
        Some("enum") => {
            let labels: Vec<String> = ty["labels"].as_array().into_iter().flatten().map(|l| format!("'{}'", l.as_str().unwrap_or(""))).collect();
            if labels.is_empty() { ty["name"].as_str().unwrap_or("enum").to_string() } else { labels.join(" | ") }
        }
        Some("array") => {
            let inner = render(&ty["of"]);
            match ty["nullable_items"].as_bool() {
                Some(true) => format!("({inner}?)[]"),
                Some(false) => format!("({inner}!)[]"),
                None => format!("{inner}[]"),
            }
        }
        Some("object") => fields(&ty["fields"]),
        Some("composite") => format!("{}{}", ty["name"].as_str().unwrap_or("record"), fields(&ty["fields"])),
        Some("list") => format!("[{}{}]", render(&ty["of"]), if ty["nullable_items"].as_bool().unwrap_or(false) { "?" } else { "" }),
        Some("json") => "json".into(),
        Some("unknown") => "unknown".into(),
        _ => ty.to_string(),
    }
}
