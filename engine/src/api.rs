//! The request and response vocabulary shared with `src/engine.ts`.
//!
//! `analyze` is a replay: the host calls it with the answers it has, the engine either returns
//! the analysis or the list of things it still needs — statements to describe, catalog queries
//! to run — and the host calls again with those answered. The engine is pure, so every round
//! starts from the statement text and the answers; nothing is kept between calls.

use crate::dialect::Dialect;
use crate::lex;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Request {
    /// Round-trip check: answers with the engine version.
    Ping,
    /// Placeholders and directives only; no database.
    Lex { sql: String, dialect: Dialect },
    /// The whole analysis, given what the host has answered so far.
    Analyze {
        sql: String,
        dialect: Dialect,
        #[serde(default)]
        answers: HashMap<String, Answer>,
    },
}

/// What the host learned for one need, keyed by the need's id.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Answer {
    Describe { describe: RawDescribe },
    Mysql { mysql: MysqlDescribe },
    Rows { rows: Vec<Vec<Value>> },
    Error { error: DbError },
}

/// What SQLite's prepare reports: per result column its name, the declared type of the table
/// column it comes from (none for an expression), and that table and column.
/// MySQL's prepare metadata as mysql2 reports it: the protocol type code, the flags (bit 1 is
/// NOT NULL) and the charset (63 is binary) per result column, and the server's type guess per `?`.
#[derive(Debug, Clone, Deserialize)]
pub struct MysqlDescribe {
    pub columns: Vec<MysqlColumn>,
    pub params: Vec<MysqlParam>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MysqlColumn {
    pub name: String,
    pub r#type: u32,
    pub flags: u32,
    #[serde(default)]
    pub charset: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MysqlParam {
    pub r#type: u32,
    #[serde(default)]
    pub flags: u32,
    #[serde(default)]
    pub charset: u32,
}

/// A Parse/Describe result as the driver reports it: parameter type oids in order, and result
/// columns with their type and, for a plain column, the table oid and attribute number.
#[derive(Debug, Clone, Deserialize)]
pub struct RawDescribe {
    pub params: Vec<u32>,
    pub columns: Vec<RawColumn>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawColumn {
    pub name: String,
    pub oid: u32,
    #[serde(default)]
    pub table: u32,
    #[serde(default)]
    pub attnum: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbError {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The 1-based character position the server reported, in the text it was given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Need {
    /// Prepare `sql` and report its parameter types and result columns.
    Describe { id: String, sql: String },
    /// Run `sql` with `params` and hand back the rows as arrays of values.
    Query { id: String, sql: String, params: Vec<Value>, scope: Scope },
}

/// How long an answer stays true: a session answer is about the database as a whole and the
/// host may keep it across statements; a statement one is about this statement's relations.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Session,
    Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Diagnostic {
    pub message: String,
    /// Byte offset in the original text.
    pub start: usize,
    pub length: usize,
    pub code: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
    /// The catalog's name for the type, for messages.
    pub type_name: String,
    /// Whether the caller may pass null: the statement tests the parameter for null or
    /// coalesces it, or it is the whole value written to a nullable column.
    #[serde(default)]
    pub nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
    pub nullable: bool,
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Analysis {
    /// `select`, `insert`, `update`, `delete`, `merge`; empty when the statement did not parse.
    pub kind: String,
    pub params: Vec<Param>,
    pub columns: Vec<Column>,
    /// `one`, `maybeOne`, `many`, or `none` for a statement that returns no rows.
    pub cardinality: String,
    /// Why the proof stopped short of `one`, when it did.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality_reason: Option<String>,
    pub rewritten: lex::Rewritten,
    pub style: lex::Style,
    pub directives: Vec<lex::Directive>,
    pub diagnostics: Vec<Diagnostic>,
    /// Paths inside JSON columns the engine could not type, for a message.
    pub unresolved: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum Response {
    Ok { version: String, fingerprint: String },
    Lexed { lexed: lex::Lexed },
    Need { needs: Vec<Need> },
    Analysis { analysis: Analysis },
    Error { message: String },
}

/// One round of the replay, through the server's `lando` extension where there is one
/// ([`crate::extension`]): the whole analysis from a server with this engine inside, a round's
/// needs in one query from a server with another build, and the needs as they are otherwise.
fn analyze(sql: &str, dialect: Dialect, answers: &HashMap<String, Answer>) -> Response {
    use crate::extension::{self, Server};
    let host = crate::host::Host::new(answers);
    // What the server offers is settled before the statement asks for anything: were the
    // statement's own needs asked in the same round, a statement that settles in one round would
    // be done before the answer came, and the extension would never be used.
    let server = if dialect == Dialect::Postgres { extension::server(&host) } else { Some(Server::Absent) };
    let Some(server) = server else { return Response::Need { needs: host.needs() } };
    if server == Server::Same {
        match extension::analyze(sql, &host) {
            Some(Some(analysis)) => return Response::Analysis { analysis },
            Some(None) => {}
            None => return Response::Need { needs: host.needs() },
        }
    }
    match crate::postgres::analyze(sql, dialect, &host) {
        Some(analysis) => Response::Analysis { analysis },
        None => {
            let needs = host.needs();
            let bundled = matches!(server, Server::Same | Server::Other { answers: true }) && !extension::bundle_failed(&needs, answers);
            Response::Need { needs: if bundled { extension::bundle(needs) } else { needs } }
        }
    }
}

pub fn call(request: &str) -> String {
    let response = match serde_json::from_str::<Request>(request) {
        Ok(Request::Ping) => Response::Ok { version: env!("CARGO_PKG_VERSION").to_string(), fingerprint: crate::FINGERPRINT.to_string() },
        Ok(Request::Lex { sql, dialect }) => match lex::lex(&sql, dialect) {
            Ok(lexed) => Response::Lexed { lexed },
            Err(e) => Response::Error { message: e.message },
        },
        Ok(Request::Analyze { sql, dialect, mut answers }) => {
            crate::extension::unbundle(&mut answers);
            analyze(&sql, dialect, &answers)
        }
        Err(e) => Response::Error { message: format!("bad request: {e}") },
    };
    serde_json::to_string(&response).expect("response serializes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extension::{ANALYZE_QUERY, PROBE_QUERY, VERSION_QUERY};
    use serde_json::json;

    fn need_ids(response: &Response) -> Vec<String> {
        match response {
            Response::Need { needs } => needs.iter().map(|n| match n {
                Need::Describe { id, .. } | Need::Query { id, .. } => id.clone(),
            }).collect(),
            _ => panic!("a round of needs"),
        }
    }

    #[test]
    fn the_server_is_settled_before_the_statement_asks_for_anything() {
        let sql = "select 1";
        let mut answers: HashMap<String, Answer> = HashMap::new();
        // The first round of a session is the probe alone.
        assert_eq!(need_ids(&analyze(sql, Dialect::Postgres, &answers)), vec![format!("query:{PROBE_QUERY}|[]")]);
        // Without the extension, the statement's own needs follow, unbundled.
        answers.insert(format!("query:{PROBE_QUERY}|[]"), Answer::Rows { rows: vec![] });
        let ids = need_ids(&analyze(sql, Dialect::Postgres, &answers));
        assert!(ids.iter().any(|id| id == &format!("describe:{sql}")), "{ids:?}");
        assert!(ids.iter().all(|id| !id.contains("lando.")), "{ids:?}");
        // With the same build inside the server, the statement is one query.
        answers.insert(format!("query:{PROBE_QUERY}|[]"), Answer::Rows { rows: vec![vec![json!("0.1.0")]] });
        assert_eq!(need_ids(&analyze(sql, Dialect::Postgres, &answers)), vec![format!("query:{VERSION_QUERY}|[]")]);
        answers.insert(format!("query:{VERSION_QUERY}|[]"), Answer::Rows { rows: vec![vec![json!({ "fingerprint": crate::FINGERPRINT })]] });
        let ids = need_ids(&analyze(sql, Dialect::Postgres, &answers));
        assert_eq!(ids.len(), 1);
        assert!(ids[0].starts_with(&format!("query:{ANALYZE_QUERY}|")), "{ids:?}");
    }
}
