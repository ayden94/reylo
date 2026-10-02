//! The project file, `.porg.yaml`: the settings the content mapper reads now and PorgREST and
//! PorgAPI read later, parsed once here so every host reads the same YAML the same way.
//!
//! A host finds the file (beside the project's `package.json`), reads it and hands the text over;
//! what the file names outside itself (the environment, `.env` files, the database) is the host's
//! to resolve, and a `${VAR}` in a URL stays as written. There is no dialect key: the URL's scheme
//! names the dialect. A key the library does not know is an error, so a typo does not pass for a
//! default; the sections of the two servers (`rest`, `api`) are accepted as they are and read by
//! those servers. `cli` says which files `porg generate` analyzes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// The connection, usually `${DATABASE_URL}`, which is also the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A connection that bypasses a pooler, which the type check prefers to `url`: describing a
    /// statement through a transaction pooler can fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_url: Option<String>,
    /// The `.env` files a development tool reads, relative to the project file; the first that
    /// sets a variable wins, and the process environment wins over all of them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Whether a parameter the statement does not type is an error (`true`) or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(default)]
    pub types: Types,
    #[serde(default, skip_serializing_if = "Cli::is_empty")]
    pub cli: Cli,
    #[serde(default, skip_serializing)]
    pub rest: Option<serde_yaml::Value>,
    #[serde(default, skip_serializing)]
    pub api: Option<serde_yaml::Value>,
}

/// What the project's driver returns for the values no JSON type holds exactly. Porg declares
/// these types; the driver's own settings decide the values.
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Types {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<Dates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bigint: Option<Bigint>,
}

/// The files `porg generate` analyzes, as globs relative to the project file: `include` replaces
/// the default (every `.sql` file), and `exclude` adds to what is always left out (dependencies,
/// build output, directories whose name starts with a dot, and what git ignores).
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cli {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

impl Cli {
    fn is_empty(&self) -> bool {
        self.include.is_none() && self.exclude.is_none()
    }
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
    // An empty file, or one holding only comments, is a project with every default.
    if text.lines().all(|l| l.trim().is_empty() || l.trim_start().starts_with('#')) {
        return Ok(Project::default());
    }
    serde_yaml::from_str(text).map_err(|e| {
        let start = e.location().map(|l| l.index()).unwrap_or(0);
        // The message carries its own " at line L column C"; the host places the error itself.
        let message = e.to_string();
        let message = match message.rfind(" at line ") {
            Some(i) if message[i..].contains(" column ") => message[..i].to_string(),
            _ => message,
        };
        let end = text[start.min(text.len())..].find(['\n', ':']).map_or(text.len(), |n| start + n);
        ConfigError { message: message.trim_end().to_string(), start, end }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_library_keys_and_leaves_the_later_sections_alone() {
        let p = parse("url: ${DATABASE_URL}\ndirect_url: ${DIRECT_URL}\nenv: [.env]\nstrict: true\ntypes:\n  dates: temporal\n  bigint: bigint\ncli:\n  include: [src/**/*.sql]\n  exclude: [src/legacy/**]\napi:\n  routes: api\nrest:\n  anything: { x: 1 }\n").unwrap();
        assert_eq!(p.url.as_deref(), Some("${DATABASE_URL}"));
        assert_eq!(p.direct_url.as_deref(), Some("${DIRECT_URL}"));
        assert_eq!(p.env, Some(vec![".env".to_string()]));
        assert_eq!(p.strict, Some(true));
        assert_eq!((p.types.dates, p.types.bigint), (Some(Dates::Temporal), Some(Bigint::Bigint)));
        assert!(p.api.is_some() && p.rest.is_some());
        // What a host reads back: the later sections are not its business.
        assert_eq!(serde_json::to_value(&p).unwrap(), serde_json::json!({ "url": "${DATABASE_URL}", "direct_url": "${DIRECT_URL}", "env": [".env"], "strict": true, "types": { "dates": "temporal", "bigint": "bigint" }, "cli": { "include": ["src/**/*.sql"], "exclude": ["src/legacy/**"] } }));
        assert_eq!(parse("").unwrap(), Project::default());
        assert_eq!(parse("# nothing yet\n").unwrap(), Project::default());
    }

    #[test]
    fn a_key_the_library_does_not_know_is_an_error_at_its_place() {
        let text = "url: postgres://x/db\ndialect: sqlite\n";
        let e = parse(text).unwrap_err();
        assert!(e.message.starts_with("unknown field `dialect`"), "{}", e.message);
        assert_eq!(&text[e.start..e.end], "dialect");
        let text = "types:\n  dates: iso\n";
        let e = parse(text).unwrap_err();
        assert!(e.message.contains("unknown variant `iso`"), "{}", e.message);
        assert!(text[e.start..].starts_with("iso"), "{e:?}");
        let e = parse("cli:\n  ignore: [x]\n").unwrap_err();
        assert!(e.message.contains("unknown field `ignore`"), "{}", e.message);
        let e = parse("strict: sometimes\n").unwrap_err();
        assert!(e.message.contains("invalid type"), "{}", e.message);
    }
}
