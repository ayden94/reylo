//! The engine's view of the database: answers the host already gave, and the needs it records
//! for the ones it has not. A lookup that misses records the need and answers `None`, and the
//! caller goes on with whatever else it can do this round, so one round collects every need
//! that does not depend on another's answer.

use crate::api::{Answer, DbError, MysqlDescribe, Need, RawDescribe, Scope};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Host<'a> {
    answers: &'a HashMap<String, Answer>,
    needs: RefCell<Vec<Need>>,
}

/// `None` is "not answered yet"; the need is recorded.
pub type Lazy<T> = Option<T>;

impl<'a> Host<'a> {
    pub fn new(answers: &'a HashMap<String, Answer>) -> Host<'a> {
        Host { answers, needs: RefCell::new(Vec::new()) }
    }

    pub fn needs(&self) -> Vec<Need> {
        self.needs.borrow().clone()
    }

    pub fn pending(&self) -> bool {
        !self.needs.borrow().is_empty()
    }

    fn record(&self, need: Need) {
        let mut needs = self.needs.borrow_mut();
        if !needs.contains(&need) {
            needs.push(need);
        }
    }

    pub fn describe(&self, sql: &str) -> Lazy<Result<RawDescribe, DbError>> {
        let id = format!("describe:{sql}");
        match self.answers.get(&id) {
            Some(Answer::Describe { describe }) => Some(Ok(describe.clone())),
            Some(Answer::Error { error }) => Some(Err(error.clone())),
            Some(Answer::Rows { .. }) | Some(Answer::Mysql { .. }) => Some(Err(DbError { message: "the host answered a describe with something else".into(), code: None, position: None })),
            None => {
                self.record(Need::Describe { id, sql: sql.to_string() });
                None
            }
        }
    }

    pub fn describe_mysql(&self, sql: &str) -> Lazy<Result<MysqlDescribe, DbError>> {
        let id = format!("describe:{sql}");
        match self.answers.get(&id) {
            Some(Answer::Mysql { mysql }) => Some(Ok(mysql.clone())),
            Some(Answer::Error { error }) => Some(Err(error.clone())),
            Some(Answer::Rows { .. }) | Some(Answer::Describe { .. }) => Some(Err(DbError { message: "the host answered a describe with something else".into(), code: None, position: None })),
            None => {
                self.record(Need::Describe { id, sql: sql.to_string() });
                None
            }
        }
    }


    pub fn query(&self, sql: &str, params: Vec<Value>, scope: Scope) -> Lazy<Result<Vec<Vec<Value>>, DbError>> {
        let id = format!("query:{sql}|{}", serde_json::to_string(&params).unwrap_or_default());
        match self.answers.get(&id) {
            Some(Answer::Rows { rows }) => Some(Ok(rows.clone())),
            Some(Answer::Error { error }) => Some(Err(error.clone())),
            Some(Answer::Describe { .. }) | Some(Answer::Mysql { .. }) => Some(Err(DbError { message: "the host answered a query with a describe".into(), code: None, position: None })),
            None => {
                self.record(Need::Query { id, sql: sql.to_string(), params, scope });
                None
            }
        }
    }
}

/// A number the driver may have decoded as a JSON number or, for `int8`, as a string.
pub fn as_u32(v: &Value) -> Option<u32> {
    match v {
        Value::Number(n) => n.as_u64().map(|n| n as u32),
        Value::String(s) => s.parse().ok(),
        _ => None,
    }
}

pub fn as_str(v: &Value) -> Option<&str> {
    v.as_str()
}

pub fn as_bool(v: &Value) -> Option<bool> {
    match v {
        Value::Bool(b) => Some(*b),
        Value::String(s) => match s.as_str() {
            "t" | "true" => Some(true),
            "f" | "false" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

pub fn as_strings(v: &Value) -> Option<Vec<String>> {
    match v {
        Value::Array(items) => items.iter().map(|i| i.as_str().map(String::from)).collect(),
        Value::Null => None,
        _ => None,
    }
}
