//! What a statement's analysis relied on, for a deployed query to check against the database it
//! runs on: the schema freshness check (wiki/structured-params.md §6, accepted on 2026-10-15; the
//! maintainer, 2026-10-20: any change fails).
//!
//! For each table the statement reads or writes, the facts of each column it names (of every
//! column, after a `*`): the type as the catalog writes it, whether it is not null, and whether it
//! has a default; and for an insert's table, every column that needs a value. A column the
//! statement does not name is no fact, so a migration that adds a column before the code that
//! reads it leaves the query alone, as the notes ask ("unrelated schema changes, like a new column
//! it never touches, don't trip the check").
//!
//! The facts are the rows of one catalog query, all text, and the module carries the query with
//! them: the run time runs the same query through the caller's driver and compares strings, so a
//! difference is a change whatever the driver decodes.

use crate::api::{Analysis, Describe, Scope};
use crate::dialect::Dialect;
use crate::host::{Host, Lazy};
use crate::lex;
use serde::Serialize;
use serde_json::Value;

/// The tables a statement reads and writes, as its database's own parse names them: each as the
/// statement writes it (a schema-qualified name with its schema), a WITH query's name never.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Tables {
    pub read: Vec<String>,
    /// What an INSERT, an UPDATE, a DELETE or a MERGE writes.
    pub write: Vec<String>,
}

/// The facts a query relied on, and the query that reads them again.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Fresh {
    /// The catalog query, the tables' names written into it, so that it takes no parameter: a
    /// host binds it as it binds nothing at all (a catalog query of the oracle's SQLite host takes
    /// none, and a MySQL session under NO_BACKSLASH_ESCAPES reads a value written into the text
    /// differently).
    pub query: String,
    /// The facts, `[table, column, type, not null, has a default]`, all text, in the query's order.
    /// The query names each column, since a driver hands rows back as objects keyed by name.
    pub rows: Vec<Vec<String>>,
    /// The tables an insert writes, each of whose columns that needs a value is a fact.
    pub inserts: Vec<String>,
}

/// Postgres: every column of each table `{tables}` names (string literals, each a name as the
/// statement writes it, quoted as an identifier), a domain's not null and default counted as the
/// column's.
const PG_QUERY: &str = "SELECT r.name AS fresh_table, a.attname::text AS fresh_column, pg_catalog.format_type(a.atttypid, a.atttypmod) AS fresh_type, \
    (a.attnotnull OR (t.typtype = 'd' AND t.typnotnull))::text AS fresh_not_null, \
    (a.atthasdef OR a.attidentity <> '' OR (t.typtype = 'd' AND t.typdefault IS NOT NULL))::text AS fresh_default \
    FROM pg_catalog.unnest(ARRAY[{tables}]::text[]) AS r(name) \
    JOIN pg_catalog.pg_attribute a ON a.attrelid = pg_catalog.to_regclass(r.name) AND a.attnum > 0 AND NOT a.attisdropped \
    JOIN pg_catalog.pg_type t ON t.oid = a.atttypid \
    ORDER BY 1, 2";

/// MySQL: every column of each table of the session's database that `{tables}` names.
const MYSQL_QUERY: &str = "select c.table_name as fresh_table, c.column_name as fresh_column, c.column_type as fresh_type, cast(c.is_nullable = 'NO' as char) as fresh_not_null, \
    cast(c.column_default is not null or c.extra like '%auto_increment%' as char) as fresh_default \
    from information_schema.columns c where c.table_schema = database() and c.table_name in ({tables}) \
    order by 1, 2";

/// SQLite: every column of each table `{tables}` (a SELECT of their names) names; an INTEGER
/// PRIMARY KEY, the rowid, has a default.
const SQLITE_QUERY: &str = "select j.value as fresh_table, p.name as fresh_column, p.type as fresh_type, cast(p.\"notnull\" as text) as fresh_not_null, \
    cast(p.dflt_value is not null or (p.pk > 0 and lower(p.type) = 'integer' and (select count(*) from pragma_table_xinfo(j.value) k where k.pk > 0) = 1) as text) as fresh_default \
    from ({tables}) j, pragma_table_xinfo(j.value) p \
    order by 1, 2";

/// A string literal every dialect reads as written: a quote doubled. MySQL reads a backslash by
/// its session's mode, so a MySQL name with one is not checked.
fn literal(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

/// The facts of `analysis`'s statement, which reads and writes `tables`; `Some(None)` where it
/// names no table the catalog knows, or its database's parse named none.
pub fn facts<D: Describe>(analysis: &Analysis, dialect: Dialect, tables: Option<&Tables>, host: &Host<'_, D>) -> Lazy<Option<Fresh>> {
    let rewritten = &analysis.rewritten;
    let Some(parsed) = tables else { return Some(None) };
    let mut tables: Vec<String> = parsed.read.iter().chain(&parsed.write).cloned().collect();
    tables.sort();
    tables.dedup();
    if tables.is_empty() {
        return Some(None);
    }
    let names: Vec<String> = tables.iter().map(|t| if dialect == Dialect::Postgres { quoted(t) } else { t.rsplit('.').next().unwrap_or(t).to_string() }).collect();
    let inserts: Vec<String> = if analysis.kind == "insert" { parsed.write.iter().map(|t| names[tables.iter().position(|x| x == t).unwrap_or(0)].clone()).collect() } else { Vec::new() };
    if dialect == Dialect::Mysql && names.iter().any(|n| n.contains('\\')) {
        return Some(None);
    }
    let literals: Vec<String> = names.iter().map(|n| literal(n)).collect();
    let query = match dialect {
        Dialect::Postgres => PG_QUERY.replace("{tables}", &literals.join(", ")),
        Dialect::Mysql => MYSQL_QUERY.replace("{tables}", &literals.join(", ")),
        Dialect::Sqlite => SQLITE_QUERY.replace("{tables}", &literals.iter().enumerate().map(|(k, l)| if k == 0 { format!("select {l} as value") } else { format!("select {l}") }).collect::<Vec<_>>().join(" union all ")),
    };
    let rows = host.query(&query, Vec::new(), Scope::Statement)?;
    let Ok(rows) = rows else { return Some(None) };
    let (named, star) = names_in(&rewritten.sql, dialect);
    let yes = |v: &str| v == "true" || v == "1";
    let text = |v: &Value| match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    };
    let kept: Vec<Vec<String>> = rows
        .iter()
        .map(|r| r.iter().map(text).collect::<Vec<String>>())
        .filter(|r| r.len() == 5)
        .filter(|r| star || named.contains(&r[1].to_ascii_lowercase()) || (inserts.contains(&r[0]) && yes(&r[3]) && !yes(&r[4])))
        .collect();
    if kept.is_empty() {
        return Some(None);
    }
    Some(Some(Fresh { query, rows: kept, inserts }))
}

/// A table's name as Postgres's `to_regclass` reads it, each part quoted, so its case holds.
fn quoted(name: &str) -> String {
    name.split('.').map(|p| format!("\"{}\"", p.replace('"', "\"\""))).collect::<Vec<_>>().join(".")
}

/// The names the statement writes outside its strings and comments, lowercased (a quoted name
/// unquoted), and whether it selects every column of something (`*`, `t.*`, `returning *`; not
/// `count(*)`).
fn names_in(sql: &str, dialect: Dialect) -> (std::collections::HashSet<String>, bool) {
    let mut names = std::collections::HashSet::new();
    let opaque = lex::lex(sql, dialect).map(|l| l.opaque).unwrap_or_default();
    let b = sql.as_bytes();
    let mut star = false;
    let mut previous = String::new();
    let mut i = 0;
    while i < b.len() {
        if let Some(&(s, e)) = opaque.iter().find(|&&(s, _)| s == i) {
            let span = &sql[s..e];
            if span.starts_with('"') || span.starts_with('`') {
                names.insert(span[1..span.len().saturating_sub(1)].to_ascii_lowercase());
                previous = "name".into();
            }
            i = e.max(i + 1);
            continue;
        }
        let c = b[i];
        if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_' || b[i] == b'$') {
                i += 1;
            }
            let word = sql[start..i].to_ascii_lowercase();
            names.insert(word.clone());
            previous = word;
            continue;
        }
        if c == b'*' {
            if matches!(previous.as_str(), "select" | "distinct" | "all" | "," | "." | "returning") {
                star = true;
            }
        }
        if !c.is_ascii_whitespace() {
            previous = (c as char).to_string();
        }
        i += 1;
    }
    (names, star)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_star_selects_every_column_and_a_product_does_not() {
        let star = |sql: &str| names_in(sql, Dialect::Postgres).1;
        assert!(star("select * from t"));
        assert!(star("select t.*, u.id from t join u on true"));
        assert!(star("insert into t (a) values (1) returning *"));
        assert!(!star("select count(*) from t"));
        assert!(!star("select a * b from t"));
        let (names, _) = names_in("select \"Mixed\", b from t where c = 'd' -- e", Dialect::Postgres);
        for n in ["mixed", "b", "t", "c", "select", "from", "where"] {
            assert!(names.contains(n), "{n}");
        }
        assert!(!names.contains("d") && !names.contains("e"));
    }

    #[test]
    fn a_table_name_keeps_its_case() {
        assert_eq!(quoted("app.Events"), "\"app\".\"Events\"");
    }
}
