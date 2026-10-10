//! The project file, `.reylo.yaml`: the settings the content mapper reads, parsed once here so
//! every host reads the same YAML the same way. The file is shared with Reylo Framework (decision
//! 93 of its log): the framework reads the top-level `url` and `env` too, and its own settings are
//! the `api` section, which `framework/engine/src/config.rs` parses.
//!
//! A host finds the file (beside the project's `package.json`), reads it and hands the text over;
//! what the file names outside itself (the environment, `.env` files, the database) is the host's
//! to resolve, and a `${VAR}` in a URL stays as written. There is no dialect key: the URL's scheme
//! names the dialect. A key the library does not know is an error, so a typo does not pass for a
//! default; the framework's `api` section and `rest`, the section PorgREST would have read, are
//! accepted as they are, and a top-level `$schema` (the JSON Schema an editor validates the file
//! against, which `reylo schema --config` prints) is ignored. `cli` says which files `reylo
//! generate` analyzes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// The published schema an editor validates the file against; no host reads it.
    #[serde(rename = "$schema", default, skip_serializing)]
    pub schema: Option<serde_yaml::Value>,
    /// The connection, usually `${DATABASE_URL}`, which is also the default. Reylo Framework
    /// connects through it too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A connection that bypasses a pooler, which the type check prefers to `url`: describing a
    /// statement through a transaction pooler can fail. The importer's alone: the framework
    /// serves through `url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_url: Option<String>,
    /// The `.env` files the importer and Reylo Framework read, relative to the project file
    /// (`.env.local`, then `.env`, when absent); the first that sets a variable wins, and the
    /// process environment wins over all of them.
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
    /// Reylo Framework's settings, which the framework parses.
    #[serde(default, skip_serializing)]
    pub api: Option<serde_yaml::Value>,
}

/// What the project's driver returns for the values drivers disagree on. Reylo declares these
/// types; the driver's own settings decide the values. Each key overrides the defaults of the
/// driver the host found in the project's `package.json`, and means the same on every dialect.
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Types {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<Dates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bigint: Option<Bigint>,
    /// Postgres `numeric`, MySQL `DECIMAL`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal: Option<Decimal>,
    /// A JSON value: parsed by the driver, or the text the database wrote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Json>,
    /// Postgres `bytea`, MySQL `BINARY` and `BLOB`, SQLite `BLOB`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<Binary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Decimal {
    String,
    Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Json {
    Parsed,
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Binary {
    Uint8Array,
    Buffer,
    ArrayBuffer,
    #[serde(rename = "number[]")]
    NumberArray,
}

/// The files `reylo generate` analyzes, as globs relative to the project file: `include` replaces
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
    let masked = mask_placeholders(text);
    let mut project: Project = serde_yaml::from_str(&masked).map_err(|e| {
        let start = e.location().map(|l| l.index()).unwrap_or(0);
        // The message carries its own " at line L column C"; the host places the error itself.
        let message = e.to_string();
        let message = match message.rfind(" at line ") {
            Some(i) if message[i..].contains(" column ") => message[..i].to_string(),
            _ => message,
        };
        let end = text[start.min(text.len())..].find(['\n', ':']).map_or(text.len(), |n| start + n);
        ConfigError { message: unmask_placeholders(message.trim_end()), start, end }
    })?;
    if masked != text {
        project.url = project.url.as_deref().map(unmask_placeholders);
        project.direct_url = project.direct_url.as_deref().map(unmask_placeholders);
        for list in [&mut project.env, &mut project.cli.include, &mut project.cli.exclude].into_iter().flatten() {
            for item in list.iter_mut() {
                *item = unmask_placeholders(item);
            }
        }
    }
    Ok(project)
}

/// The text with each `${VAR}` spelled `$<VAR>`. A bare `${VAR}` inside a flow mapping or
/// sequence is not YAML (its `{` opens a nested mapping), and the framework's `api` section is
/// often written so (`jwt: { jwks: ${JWKS_URL} }`): the framework interpolates before it parses,
/// but the importer keeps a URL's `${VAR}` as written. The spelling has the same length, so an
/// error's offsets still point into the text, and `unmask_placeholders` turns it back in what
/// the parse returns. A text that already holds `$<` is parsed as written.
fn mask_placeholders(text: &str) -> std::borrow::Cow<'_, str> {
    if text.contains("$<") || !text.contains("${") {
        return std::borrow::Cow::Borrowed(text);
    }
    std::borrow::Cow::Owned(respell(text, "${", '}', "$<", '>'))
}

fn unmask_placeholders(text: &str) -> String {
    respell(text, "$<", '>', "${", '}')
}

/// Each `<open>NAME<close>` with a variable name between, spelled with the other delimiters.
fn respell(text: &str, open: &str, close: char, to_open: &str, to_close: char) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(open) {
        out.push_str(&rest[..at]);
        let after = &rest[at + open.len()..];
        let name_len = after.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(after.len());
        if name_len > 0 && after[name_len..].starts_with(close) {
            out.push_str(to_open);
            out.push_str(&after[..name_len]);
            out.push(to_close);
            rest = &after[name_len + close.len_utf8()..];
        } else {
            out.push_str(open);
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_library_keys_and_leaves_the_later_sections_alone() {
        let p = parse("url: ${DATABASE_URL}\ndirect_url: ${DIRECT_URL}\nenv: [.env]\nstrict: true\ntypes:\n  dates: temporal\n  bigint: bigint\n  decimal: number\n  json: string\n  binary: number[]\ncli:\n  include: [src/**/*.sql]\n  exclude: [src/legacy/**]\napi:\n  routes: api\nrest:\n  anything: { x: 1 }\n").unwrap();
        assert_eq!(p.url.as_deref(), Some("${DATABASE_URL}"));
        assert_eq!(p.direct_url.as_deref(), Some("${DIRECT_URL}"));
        assert_eq!(p.env, Some(vec![".env".to_string()]));
        assert_eq!(p.strict, Some(true));
        assert_eq!((p.types.dates, p.types.bigint), (Some(Dates::Temporal), Some(Bigint::Bigint)));
        assert!(p.api.is_some() && p.rest.is_some());
        // What a host reads back: the later sections are not its business.
        assert_eq!(serde_json::to_value(&p).unwrap(), serde_json::json!({ "url": "${DATABASE_URL}", "direct_url": "${DIRECT_URL}", "env": [".env"], "strict": true, "types": { "dates": "temporal", "bigint": "bigint", "decimal": "number", "json": "string", "binary": "number[]" }, "cli": { "include": ["src/**/*.sql"], "exclude": ["src/legacy/**"] } }));
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

    #[test]
    fn a_framework_project_file_reads_with_its_schema_and_api_section() {
        let text = "$schema: https://example.test/reylo.schema.json\nurl: ${DATABASE_URL}\nenv: [.env.local, .env]\ntypes: { dates: temporal }\napi:\n  authenticators:\n    clerk:\n      jwt:\n        jwks: ${JWKS_URL}\n  codegen:\n    typescript: { out: web/src/api }\n";
        let p = parse(text).unwrap();
        assert_eq!(p.url.as_deref(), Some("${DATABASE_URL}"));
        assert_eq!(p.types.dates, Some(Dates::Temporal));
        assert!(p.schema.is_some() && p.api.is_some());
        // Neither `$schema` nor `api` is the host's to read back.
        assert_eq!(serde_json::to_value(&p).unwrap(), serde_json::json!({ "url": "${DATABASE_URL}", "env": [".env.local", ".env"], "types": { "dates": "temporal" } }));
    }

    #[test]
    fn a_placeholder_in_a_flow_mapping_parses_and_reads_back_as_written() {
        let text = "url: ${DATABASE_URL}\ndirect_url: \"${DIRECT_URL}\"\nenv: [${ENV_FILE}, .env]\napi:\n  authenticators:\n    clerk:\n      jwt: { from: { header: Authorization, scheme: Bearer }, jwks: ${JWKS_URL}, issuer: ${ISSUER} }\n";
        let p = parse(text).unwrap();
        assert_eq!(p.url.as_deref(), Some("${DATABASE_URL}"));
        assert_eq!(p.direct_url.as_deref(), Some("${DIRECT_URL}"));
        assert_eq!(p.env, Some(vec!["${ENV_FILE}".to_string(), ".env".to_string()]));
        assert_eq!(p.api.unwrap()["authenticators"]["clerk"]["jwt"]["jwks"].as_str(), Some("$<JWKS_URL>"), "the api section is the framework's to read");
        // The offsets of an error still point into the text as written.
        let text = "url: ${DATABASE_URL}\napi: { a: ${X} }\ndialect: x\n";
        let e = parse(text).unwrap_err();
        assert_eq!(&text[e.start..e.end], "dialect");
        // A text that writes `$<` itself is read as written.
        assert!(parse("url: $<x>\napi: { a: ${X} }\n").is_err());
    }
}
