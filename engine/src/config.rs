//! The project file, `.porg.toml`: the settings the content mapper reads now and PorgREST and
//! PorgAPI read later, parsed once here so every host reads the same TOML the same way.
//!
//! A host finds the file (walking up from its project directory), reads it and hands the text
//! over; what the file names outside itself (the environment, `.env` files, the database) is the
//! host's to resolve, and a `${VAR}` in `url` stays as written. A key the library does not know is
//! an error, so a typo does not pass for a default; the tables of the later pieces (`[rest]`,
//! `[api]`, `[check]`, `[auth]`, `[live]`) are accepted as they are and read by those pieces.

use crate::dialect::Dialect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// The dialect; without it the URL's scheme decides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<Dialect>,
    /// The connection, usually `${DATABASE_URL}`, which is also the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The `.env` files a development tool reads, relative to the project file; the first that
    /// sets a variable wins, and the process environment wins over all of them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_files: Option<Vec<String>>,
    #[serde(default)]
    pub types: Types,
    #[serde(default, skip_serializing)]
    pub rest: Option<toml::Table>,
    #[serde(default, skip_serializing)]
    pub api: Option<toml::Table>,
    #[serde(default, skip_serializing)]
    pub check: Option<toml::Table>,
    #[serde(default, skip_serializing)]
    pub auth: Option<toml::Table>,
    #[serde(default, skip_serializing)]
    pub live: Option<toml::Table>,
}

/// How the library represents the values no JSON type holds exactly.
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Types {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<Dates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bigint: Option<Bigint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Dates {
    #[serde(rename = "string")]
    String,
    Date,
    #[serde(rename = "temporal")]
    Temporal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bigint {
    String,
    Number,
    Bigint,
}

/// Why the file does not parse, at byte offsets in its text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

pub fn parse(text: &str) -> Result<Project, ConfigError> {
    toml::from_str(text).map_err(|e| {
        let span = e.span().unwrap_or(0..0);
        ConfigError { message: e.message().trim_end().to_string(), start: span.start, end: span.end }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_library_keys_and_leaves_the_later_tables_alone() {
        let p = parse("dialect = \"sqlite\"\nurl = \"${DATABASE_URL}\"\nenv_files = [\".env\"]\n[types]\ndates = \"temporal\"\nbigint = \"bigint\"\n[api]\nroutes = \"api\"\n[rest.anything]\nx = 1\n").unwrap();
        assert_eq!(p.dialect, Some(Dialect::Sqlite));
        assert_eq!(p.url.as_deref(), Some("${DATABASE_URL}"));
        assert_eq!(p.env_files, Some(vec![".env".to_string()]));
        assert_eq!((p.types.dates, p.types.bigint), (Some(Dates::Temporal), Some(Bigint::Bigint)));
        assert!(p.api.is_some() && p.rest.is_some());
        // What a host reads back: the later tables are not its business.
        assert_eq!(serde_json::to_value(&p).unwrap(), serde_json::json!({ "dialect": "sqlite", "url": "${DATABASE_URL}", "env_files": [".env"], "types": { "dates": "temporal", "bigint": "bigint" } }));
        assert_eq!(parse("").unwrap(), Project::default());
    }

    #[test]
    fn a_key_the_library_does_not_know_is_an_error_at_its_place() {
        let text = "dialect = \"postgres\"\ndialct = \"sqlite\"\n";
        let e = parse(text).unwrap_err();
        assert!(e.message.starts_with("unknown field `dialct`"), "{}", e.message);
        assert_eq!(&text[e.start..e.end], "dialct");
        let text = "[types]\ndates = \"iso\"\n";
        let e = parse(text).unwrap_err();
        assert!(e.message.contains("unknown variant `iso`"), "{}", e.message);
        assert_eq!(&text[e.start..e.end], "\"iso\"");
        let e = parse("url = \n").unwrap_err();
        assert!(e.start <= 6, "{e:?}");
    }
}
