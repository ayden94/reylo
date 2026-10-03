//! The dialects the engine reads. Every statement is parsed with libpg_query; SQLite and MySQL
//! statements are read in the SQL they share with Postgres, after their placeholders become `$n`,
//! and what does not parse there proves nothing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    Postgres,
    Sqlite,
    Mysql,
}

impl Dialect {
    pub fn name(self) -> &'static str {
        match self {
            Dialect::Postgres => "postgres",
            Dialect::Sqlite => "sqlite",
            Dialect::Mysql => "mysql",
        }
    }
}
