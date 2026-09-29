//! Placeholders and directives, read off the raw statement text.
//!
//! Lando lets a statement name its parameters in any dialect: `:name`, `$name` and `@name` are
//! named placeholders, `$1`, `?` and `?1` positional ones, and one statement uses one style. A
//! name may be dotted (`:auth.user_id`): each `.segment` starts like a name, so `arr[1:2]` and
//! `:t.*` end where they should. The engine rewrites placeholders to whatever the database binds
//! by position (`$n` on Postgres, `?` on MySQL, `?n` for the SQLite prepare, where SQLite's own
//! `:name` would end at the first dot) and keeps a map back to the author's text, so a server
//! error at a rewritten position lands on the right characters of the `.sql` file.
//!
//! The pass knows what the parser knows about text: single-quoted literals (with `''`), `E''`
//! escape strings (with backslashes), dollar quoting, double-quoted identifiers, line comments,
//! nested block comments, and `::` casts. Two readings are per dialect: `?` is a placeholder
//! everywhere but Postgres, where it is the `jsonb ? text` operator; `$n` is positional
//! everywhere, and a Postgres `$tag$…$tag$` string is read before a `$name` parameter is. The
//! spans it skipped as text are returned as [`Lexed::opaque`], so a caller that looks for its own
//! construct in the statement does not need a scanner of its own.
//!
//! Directives are `-- @name value` line comments; the engine collects them and the caller
//! decides what each means.

use crate::dialect::Dialect;
use serde::Serialize;

/// One placeholder occurrence, by byte offsets in the original text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Placeholder {
    /// The parameter name: the author's for a named placeholder (dots included), `$n` for a
    /// numbered one (`$3` and SQLite's `?3` alike, so both name one position), and `?k` for the
    /// k-th bare `?`.
    pub name: String,
    pub start: usize,
    pub end: usize,
}

/// `-- @name value`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Directive {
    pub name: String,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    None,
    Named,
    Positional,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Lexed {
    /// Distinct parameter names in first-appearance order: the binding order.
    pub params: Vec<String>,
    pub placeholders: Vec<Placeholder>,
    pub directives: Vec<Directive>,
    pub style: Style,
    /// Every span the walk read as text rather than code, `(start, end)` in byte offsets, in
    /// source order and never overlapping: string literals (an `E''` string from its `E`), dollar
    /// quotes, quoted identifiers, line comments (up to their newline) and block comments.
    pub opaque: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LexError {
    pub message: String,
    pub offset: usize,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (at byte {})", self.message, self.offset)
    }
}

impl std::error::Error for LexError {}

/// The placeholder syntax a rewrite emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// `$n`, numbered by binding order.
    Dollar,
    /// `?`, one per occurrence, in text order; the binding order repeats a name where it repeats.
    Question,
    /// `?n`, numbered by binding order (the parameter's 1-based index in [`Lexed::params`]), so a
    /// repeated name is one position. Unlike [`Target::Dollar`], an author's `$3` is renumbered
    /// too: the binding order is `params` either way, and SQLite reads `?n` as position `n`.
    Numbered,
}

/// A rewritten statement plus the map back to the author's text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Rewritten {
    pub sql: String,
    /// The binding order: for `?` one entry per occurrence, for `$n` and `?n` one per distinct name.
    pub order: Vec<String>,
    /// Per placeholder in text order: `(original start, original end, rewritten start, rewritten end)`.
    pub spans: Vec<(usize, usize, usize, usize)>,
}

impl Rewritten {
    /// The byte offset in the original text that a byte offset in the rewritten text refers to.
    pub fn to_original(&self, pos: usize) -> usize {
        let mut delta: isize = 0;
        for &(os, oe, rs, re) in &self.spans {
            if pos >= re {
                delta += (oe - os) as isize - (re - rs) as isize;
            } else if pos >= rs {
                return os;
            } else {
                break;
            }
        }
        (pos as isize + delta).max(0) as usize
    }
}

fn is_name_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

fn prev_is_name_char(b: &[u8], i: usize) -> bool {
    i > 0 && is_name_char(b[i - 1])
}

fn name_end(b: &[u8], start: usize) -> usize {
    let mut j = start;
    while j < b.len() && is_name_char(b[j]) {
        j += 1;
    }
    j
}

/// A placeholder name from `start`: a name, then any `.segment` that starts like one
/// (`:auth.user_id`). `:t.*` stops before the dot.
fn dotted_name_end(b: &[u8], start: usize) -> usize {
    let mut j = name_end(b, start);
    while b.get(j) == Some(&b'.') && b.get(j + 1).is_some_and(|c| is_name_start(*c)) {
        j = name_end(b, j + 1);
    }
    j
}

fn digits_end(b: &[u8], start: usize) -> usize {
    let mut j = start;
    while j < b.len() && b[j].is_ascii_digit() {
        j += 1;
    }
    j
}

/// `$tag$` at `i`: the offset one past its closing `$`, when `i` starts a dollar-quote opener.
fn dollar_tag_end(b: &[u8], i: usize) -> Option<usize> {
    if b.get(i) != Some(&b'$') {
        return None;
    }
    let mut j = i + 1;
    if j < b.len() && is_name_start(b[j]) {
        j = name_end(b, j);
    }
    (b.get(j) == Some(&b'$')).then_some(j + 1)
}

fn find_bytes(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() || from >= hay.len() {
        return None;
    }
    hay[from..].windows(needle.len()).position(|w| w == needle).map(|p| p + from)
}

pub fn lex(sql: &str, dialect: Dialect) -> Result<Lexed, LexError> {
    let b = sql.as_bytes();
    let mut i = 0usize;
    let mut params: Vec<String> = Vec::new();
    let mut placeholders: Vec<Placeholder> = Vec::new();
    let mut directives: Vec<Directive> = Vec::new();
    let mut opaque: Vec<(usize, usize)> = Vec::new();
    let mut questions = 0usize;
    let mut push = |name: String, start: usize, end: usize, params: &mut Vec<String>| {
        if !params.contains(&name) {
            params.push(name.clone());
        }
        placeholders.push(Placeholder { name, start, end });
    };
    while i < b.len() {
        let c = b[i];
        match c {
            b'\'' => {
                let start = i;
                i += 1;
                loop {
                    match b.get(i) {
                        None => return Err(LexError { message: "unterminated string literal".into(), offset: i }),
                        Some(b'\'') if b.get(i + 1) == Some(&b'\'') => i += 2,
                        Some(b'\'') => {
                            i += 1;
                            break;
                        }
                        Some(_) => i += 1,
                    }
                }
                opaque.push((start, i));
            }
            b'E' | b'e' if b.get(i + 1) == Some(&b'\'') && !prev_is_name_char(b, i) => {
                let start = i;
                i += 2;
                loop {
                    match b.get(i) {
                        None => return Err(LexError { message: "unterminated escape string".into(), offset: i }),
                        Some(b'\\') => i += 2,
                        Some(b'\'') if b.get(i + 1) == Some(&b'\'') => i += 2,
                        Some(b'\'') => {
                            i += 1;
                            break;
                        }
                        Some(_) => i += 1,
                    }
                }
                opaque.push((start, i));
            }
            b'"' | b'`' => {
                let quote = c;
                let start = i;
                i += 1;
                loop {
                    match b.get(i) {
                        None => return Err(LexError { message: "unterminated quoted identifier".into(), offset: i }),
                        Some(&q) if q == quote && b.get(i + 1) == Some(&quote) => i += 2,
                        Some(&q) if q == quote => {
                            i += 1;
                            break;
                        }
                        Some(_) => i += 1,
                    }
                }
                opaque.push((start, i));
            }
            b'-' if b.get(i + 1) == Some(&b'-') => {
                let start = i;
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                opaque.push((start, i));
                let text = sql[start + 2..i].trim();
                if let Some(rest) = text.strip_prefix('@') {
                    let name_len = rest.bytes().take_while(|c| is_name_char(*c)).count();
                    if name_len > 0 {
                        directives.push(Directive {
                            name: rest[..name_len].to_string(),
                            value: rest[name_len..].trim().to_string(),
                            start,
                            end: i,
                        });
                    }
                }
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let start = i;
                let mut depth = 1usize;
                i += 2;
                while depth > 0 {
                    match (b.get(i), b.get(i + 1)) {
                        (Some(b'/'), Some(b'*')) => {
                            depth += 1;
                            i += 2;
                        }
                        (Some(b'*'), Some(b'/')) => {
                            depth -= 1;
                            i += 2;
                        }
                        (Some(_), _) => i += 1,
                        (None, _) => return Err(LexError { message: "unterminated block comment".into(), offset: i }),
                    }
                }
                opaque.push((start, i));
            }
            b':' => {
                if b.get(i + 1) == Some(&b':') {
                    // A `::` cast: the type name after it is not a parameter.
                    i += 2;
                    if i < b.len() && is_name_start(b[i]) {
                        i = name_end(b, i);
                    }
                    continue;
                }
                match b.get(i + 1) {
                    Some(&n) if is_name_start(n) && !prev_is_name_char(b, i) => {
                        let end = dotted_name_end(b, i + 1);
                        push(sql[i + 1..end].to_string(), i, end, &mut params);
                        i = end;
                    }
                    _ => i += 1,
                }
            }
            b'$' => {
                if dialect == Dialect::Postgres {
                    if let Some(tag_end) = dollar_tag_end(b, i) {
                        let tag = &b[i..tag_end];
                        match find_bytes(b, tag, tag_end) {
                            Some(close) => {
                                opaque.push((i, close + tag.len()));
                                i = close + tag.len();
                                continue;
                            }
                            None => {
                                return Err(LexError {
                                    message: format!("unterminated dollar-quoted string {}", sql[i..tag_end].to_string()),
                                    offset: i,
                                })
                            }
                        }
                    }
                }
                match b.get(i + 1) {
                    Some(&n) if is_name_start(n) => {
                        let end = dotted_name_end(b, i + 1);
                        push(sql[i + 1..end].to_string(), i, end, &mut params);
                        i = end;
                    }
                    Some(&n) if n.is_ascii_digit() => {
                        let end = digits_end(b, i + 1);
                        push(sql[i..end].to_string(), i, end, &mut params);
                        i = end;
                    }
                    _ => i += 1,
                }
            }
            b'@' => match b.get(i + 1) {
                Some(&n) if is_name_start(n) && !prev_is_name_char(b, i) => {
                    let end = dotted_name_end(b, i + 1);
                    push(sql[i + 1..end].to_string(), i, end, &mut params);
                    i = end;
                }
                _ => i += 1,
            },
            b'?' if dialect != Dialect::Postgres => {
                if b.get(i + 1).is_some_and(u8::is_ascii_digit) {
                    // SQLite's `?NNN` names position NNN, as `$NNN` does, so it takes that name.
                    let end = digits_end(b, i + 1);
                    push(format!("${}", &sql[i + 1..end]), i, end, &mut params);
                    i = end;
                } else {
                    questions += 1;
                    push(format!("?{questions}"), i, i + 1, &mut params);
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    let positional = params.iter().filter(|p| is_positional(p)).count();
    let style = match (positional, params.len()) {
        (_, 0) => Style::None,
        (0, _) => Style::Named,
        (p, n) if p == n => Style::Positional,
        _ => Style::Mixed,
    };
    Ok(Lexed { params, placeholders, directives, style, opaque })
}

/// Whether a parameter name is one the lexer minted for a positional placeholder.
pub fn is_positional(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('$') | Some('?')) && chars.as_str().bytes().all(|c| c.is_ascii_digit()) && name.len() > 1
}

/// Rewrite every placeholder to the target's syntax.
pub fn rewrite(sql: &str, lexed: &Lexed, target: Target) -> Rewritten {
    rewrite_with(sql, lexed, target, &|_, rendered| rendered.to_string())
}

/// Rewrite with a hook over each rendered placeholder — a probe pins a cast on it.
pub fn rewrite_with(sql: &str, lexed: &Lexed, target: Target, decorate: &dyn Fn(&str, &str) -> String) -> Rewritten {
    let mut out = String::with_capacity(sql.len());
    let mut spans = Vec::with_capacity(lexed.placeholders.len());
    let mut order: Vec<String> = Vec::new();
    let mut last = 0;
    for p in &lexed.placeholders {
        let rendered = match target {
            Target::Dollar => {
                if let Some(n) = p.name.strip_prefix('$') {
                    format!("${n}")
                } else {
                    format!("${}", lexed.params.iter().position(|q| q == &p.name).map_or(0, |i| i + 1))
                }
            }
            Target::Question => "?".to_string(),
            Target::Numbered => format!("?{}", lexed.params.iter().position(|q| q == &p.name).map_or(0, |i| i + 1)),
        };
        let rendered = decorate(&p.name, &rendered);
        out.push_str(&sql[last..p.start]);
        let rs = out.len();
        out.push_str(&rendered);
        spans.push((p.start, p.end, rs, out.len()));
        last = p.end;
        if target == Target::Question {
            order.push(p.name.clone());
        }
    }
    out.push_str(&sql[last..]);
    if target != Target::Question {
        order = lexed.params.clone();
    }
    Rewritten { sql: out, order, spans }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(sql: &str, d: Dialect) -> Vec<String> {
        lex(sql, d).unwrap().params
    }

    #[test]
    fn styles() {
        assert_eq!(names("select :a, @b, $c, :a", Dialect::Postgres), ["a", "b", "c"]);
        assert_eq!(names("select $1, $2, $1", Dialect::Postgres), ["$1", "$2"]);
        assert_eq!(names("select ?, ?", Dialect::Sqlite), ["?1", "?2"]);
        assert_eq!(lex("select :a, $1", Dialect::Postgres).unwrap().style, Style::Mixed);
        assert_eq!(lex("select 1", Dialect::Postgres).unwrap().style, Style::None);
    }

    #[test]
    fn skips_text_and_casts() {
        let sql = "select ':x' as a, \":y\" as b, e'\\':z' as c, $$:w$$ as d, x::int, arr[1:2], tags ? 'k' -- :c\n /* :d /* :e */ */ from t where id = :id";
        assert_eq!(names(sql, Dialect::Postgres), ["id"]);
        assert_eq!(names("select json_extract(j, '$.a') = ? and x = :n", Dialect::Sqlite), ["?1", "n"]);
        assert_eq!(names("select `a:b`, 'x' from t where a = @a", Dialect::Mysql), ["a"]);
    }

    #[test]
    fn directives() {
        let l = lex("-- @returns maybeOne\n-- @param id number\nselect :id", Dialect::Postgres).unwrap();
        assert_eq!(l.directives.len(), 2);
        assert_eq!((l.directives[0].name.as_str(), l.directives[0].value.as_str()), ("returns", "maybeOne"));
        assert_eq!((l.directives[1].name.as_str(), l.directives[1].value.as_str()), ("param", "id number"));
    }

    #[test]
    fn rewrites_and_maps_back() {
        let sql = "select * from t where id = :id and name = :name and id = :id";
        let l = lex(sql, Dialect::Postgres).unwrap();
        let r = rewrite(sql, &l, Target::Dollar);
        assert_eq!(r.sql, "select * from t where id = $1 and name = $2 and id = $1");
        assert_eq!(r.order, ["id", "name"]);
        // "name" starts at the same place in both; the third placeholder shifted.
        let third = r.sql.rfind("$1").unwrap();
        assert_eq!(r.to_original(third), sql.rfind(":id").unwrap());
        assert_eq!(r.to_original(r.sql.len()), sql.len());
        let q = rewrite(sql, &l, Target::Question);
        assert_eq!(q.sql, "select * from t where id = ? and name = ? and id = ?");
        assert_eq!(q.order, ["id", "name", "id"]);
    }

    #[test]
    fn errors() {
        assert!(lex("select 'abc", Dialect::Postgres).is_err());
        assert!(lex("select $x$abc", Dialect::Postgres).is_err());
        assert!(lex("select /* a", Dialect::Postgres).is_err());
        assert_eq!(lex("select 'abc", Dialect::Postgres).unwrap_err().to_string(), "unterminated string literal (at byte 11)");
    }

    #[test]
    fn dotted_names() {
        let sql = "select :a.b";
        let l = lex(sql, Dialect::Postgres).unwrap();
        assert_eq!(l.params, ["a.b"]);
        assert_eq!(l.placeholders, [Placeholder { name: "a.b".into(), start: 7, end: 11 }]);
        assert_eq!(l.style, Style::Named);
        assert_eq!(rewrite(sql, &l, Target::Dollar).sql, "select $1");
        assert_eq!(rewrite(sql, &l, Target::Numbered).sql, "select ?1");
        assert_eq!(rewrite(sql, &l, Target::Question).sql, "select ?");
        // Every segment starts like a name; the named forms agree; a cast after one is a cast.
        assert_eq!(names("select :auth.user_id, :org.name::text, :t.*, :x.1", Dialect::Postgres), ["auth.user_id", "org.name", "t", "x"]);
        assert_eq!(names("select $a.b, @c.d from t", Dialect::Sqlite), ["a.b", "c.d"]);
        // A dotted name survives the SQLite prepare only through the rewrite, and maps back.
        let sql = "select :a.b, :c";
        let l = lex(sql, Dialect::Sqlite).unwrap();
        let r = rewrite(sql, &l, Target::Numbered);
        assert_eq!(r.sql, "select ?1, ?2");
        assert_eq!(r.to_original(r.sql.find("?2").unwrap()), sql.find(":c").unwrap());
        assert_eq!(r.to_original(r.sql.len()), sql.len());
    }

    #[test]
    fn numbered_question_is_a_position() {
        let l = lex("select ?3, $3", Dialect::Sqlite).unwrap();
        assert_eq!(l.params, ["$3"]);
        assert_eq!(l.placeholders.iter().map(|p| (p.start, p.end)).collect::<Vec<_>>(), [(7, 9), (11, 13)]);
        assert_eq!(l.style, Style::Positional);
        assert_eq!(lex("select ?3, :x", Dialect::Sqlite).unwrap().style, Style::Mixed);
        // On Postgres `?` is still the operator, digits or not.
        assert!(lex("select j ? 'k', 1 ?2", Dialect::Postgres).unwrap().params.is_empty());
    }

    #[test]
    fn array_slices_are_not_placeholders() {
        for d in [Dialect::Postgres, Dialect::Sqlite, Dialect::Mysql] {
            assert!(lex("select arr[1:2], arr[2:], arr[1:2].x from t", d).unwrap().params.is_empty());
        }
    }

    #[test]
    fn opaque_spans() {
        let sql = "select 'a''b', \"q\"\"r\", $t$ :x $t$, e'\\'' -- c :y\n/* d /* :z */ */ :w";
        let l = lex(sql, Dialect::Postgres).unwrap();
        assert_eq!(l.params, ["w"]);
        assert_eq!(l.opaque, [(7, 13), (15, 21), (23, 33), (35, 40), (41, 48), (49, 65)]);
        let text: Vec<&str> = l.opaque.iter().map(|&(s, e)| &sql[s..e]).collect();
        assert_eq!(text, ["'a''b'", "\"q\"\"r\"", "$t$ :x $t$", "e'\\''", "-- c :y", "/* d /* :z */ */"]);
        // A directive is a line comment like any other; code between the spans is not listed.
        let l = lex("-- @returns one\nselect `a`, 1", Dialect::Mysql).unwrap();
        assert_eq!(l.opaque, [(0, 15), (23, 26)]);
    }

    #[test]
    fn numbered_target() {
        let sql = "select :a, :b, :a";
        let l = lex(sql, Dialect::Sqlite).unwrap();
        let r = rewrite(sql, &l, Target::Numbered);
        assert_eq!(r.sql, "select ?1, ?2, ?1");
        assert_eq!(r.order, ["a", "b"]);
        assert_eq!(r.spans, [(7, 9, 7, 9), (11, 13, 11, 13), (15, 17, 15, 17)]);
        // An author's numbering is renumbered by binding order: `?n` binds `params[n - 1]`.
        let sql = "select $2, ?1, $2";
        let l = lex(sql, Dialect::Sqlite).unwrap();
        let r = rewrite(sql, &l, Target::Numbered);
        assert_eq!(r.sql, "select ?1, ?2, ?1");
        assert_eq!(r.order, ["$2", "$1"]);
    }
}
