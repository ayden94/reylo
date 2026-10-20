//! What every database's analysis starts from: the statement's placeholders, and the bare analysis
//! a statement gets when nothing more can be said of it.

use crate::api::{Analysis, Diagnostic, Param};
use crate::dialect::Dialect;
use crate::lex::{self, Lexed, Style};
use crate::types::Type;

/// The statement's placeholders, or the bare analysis that reports why they cannot be read: a
/// lex error, named and positional parameters mixed, or a parameter used more than once.
pub fn lexed(sql: &str, dialect: Dialect) -> Result<Lexed, Analysis> {
    let lexed = match lex::lex(sql, dialect) {
        Ok(l) => l,
        Err(e) => return Err(bare(sql, &none(), dialect, vec![diag(e.message, e.offset, 1, 1)])),
    };
    if lexed.style == Style::Mixed {
        let first = lexed.placeholders.iter().find(|p| lex::is_positional(&p.name)).expect("a mixed statement has a positional placeholder");
        let d = diag("Named and positional parameters cannot be mixed in one statement.", first.start, first.end - first.start, 2);
        return Err(bare(sql, &lexed, dialect, vec![d]));
    }
    let repeated = repeated(sql, &lexed);
    if !repeated.is_empty() {
        return Err(bare(sql, &lexed, dialect, repeated));
    }
    Ok(lexed)
}

/// Each later use of a parameter, as the diagnostic at its place ([`lex::repeated`]).
pub fn repeated(sql: &str, lexed: &Lexed) -> Vec<Diagnostic> {
    lex::repeated(sql, lexed).into_iter().map(|r| diag(r.message, r.start, r.end - r.start, 3)).collect()
}

/// A diagnostic at least one byte long.
pub fn diag(message: impl Into<String>, start: usize, length: usize, code: u32) -> Diagnostic {
    Diagnostic { message: message.into(), start, length: length.max(1), code }
}

/// A statement with no placeholders read.
pub fn none() -> Lexed {
    Lexed { params: vec![], placeholders: vec![], style: Style::None, opaque: vec![] }
}

/// What the engine answers when it cannot describe: every parameter unknown, no columns, and
/// `many` rows.
pub fn bare(sql: &str, lexed: &Lexed, dialect: Dialect, diagnostics: Vec<Diagnostic>) -> Analysis {
    let rewritten = lex::rewrite(sql, lexed, dialect.binds());
    Analysis {
        kind: String::new(),
        params: lexed.params.iter().map(|n| Param { name: n.clone(), ty: Type::Unknown, type_name: String::new(), nullable: false, schema: None, cast: None }).collect(),
        columns: Vec::new(),
        cardinality: "many".to_string(),
        cardinality_reason: None,
        rewritten,
        style: lexed.style,
        diagnostics,
        unresolved: Vec::new(),
        structured: Vec::new(),
        fresh: None,
        tables: None,
    }
}

/// The bare analysis of `sql` with `diagnostics`, its parameters as far as the lexer reads them.
pub fn bare_of(sql: &str, dialect: Dialect, diagnostics: Vec<Diagnostic>) -> Analysis {
    let lexed = lex::lex(sql, dialect).unwrap_or_else(|_| none());
    bare(sql, &lexed, dialect, diagnostics)
}
