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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Diagnostic {
    pub message: String,
    /// Byte offset in the original text.
    pub start: usize,
    pub length: usize,
    pub code: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
    /// The catalog's name for the type, for messages.
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize)]
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
    Ok { version: String },
    Lexed { lexed: lex::Lexed },
    Need { needs: Vec<Need> },
    Analysis { analysis: Analysis },
    Error { message: String },
}

pub fn call(request: &str) -> String {
    let response = match serde_json::from_str::<Request>(request) {
        Ok(Request::Ping) => Response::Ok { version: env!("CARGO_PKG_VERSION").to_string() },
        Ok(Request::Lex { sql, dialect }) => match lex::lex(&sql, dialect) {
            Ok(lexed) => Response::Lexed { lexed },
            Err(e) => Response::Error { message: e.message },
        },
        Ok(Request::Analyze { sql, dialect, answers }) => {
            let host = crate::host::Host::new(&answers);
            match crate::postgres::analyze(&sql, dialect, &host) {
                Some(analysis) => Response::Analysis { analysis },
                None => Response::Need { needs: host.needs() },
            }
        }
        Err(e) => Response::Error { message: format!("bad request: {e}") },
    };
    serde_json::to_string(&response).expect("response serializes")
}
