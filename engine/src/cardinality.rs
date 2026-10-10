//! Static cardinality: whether a statement returns exactly one row, proven from its shape.
//!
//! A middleware returns exactly one row, and the build refuses one it cannot prove does. An
//! endpoint's `returns:` is checked against the same reading, but only for a contradiction it
//! proves — `returns: one` over `LIMIT 0` — never for a shape it merely cannot see through,
//! because `WHERE id = :id` is unprovable and correct.
//!
//! The proof is small and fail-closed: anything it does not recognise is unproven, and
//! "unproven" never means "proven otherwise". The runtime still counts every middleware's rows,
//! because the database the request runs against is not the one the build read (a function can
//! be recreated as set-returning after the build, and a prepared statement re-plans onto it).
//!
//! A `SELECT` returns exactly one row when all of these hold:
//!
//! 1. It has the shape of one row, which is [`infer::single_row`]'s reading: no set operation, no
//!    `VALUES`, no `GROUP BY` (a row per group, and none over no rows), no `HAVING` (it can drop
//!    the one group), no `LIMIT` or `OFFSET` (`LIMIT 0` and `OFFSET 1` drop the row).
//! 2. It aggregates, or it has neither a `FROM` nor a `WHERE`. An aggregate with no `GROUP BY` is
//!    one group over any input, empty included (`SELECT count(*) FROM t WHERE false` and even
//!    `SELECT count(*) WHERE false` are one row on both dialects); whether a target aggregates is
//!    [`infer::aggregates`]'s reading. Without one, a `FROM` is a row per row it matches, and a
//!    `WHERE` with no `FROM` drops the one row when it is false (`SELECT 1 WHERE :x = 1`).
//! 3. Nothing at its own level returns a set (Postgres only; see below). `SELECT
//!    generate_series(1, 3)` is three rows, `SELECT unnest('{}'::int[])` none, and an aggregate
//!    beside one is as many rows as the set (`SELECT count(*), generate_series(1, 3) FROM t`).
//!
//! A `WITH` changes none of this: the main statement reads a CTE only through a `FROM` or a
//! subquery. A subquery answers one value to the level above it whatever it returns: `EXISTS`
//! is one boolean, and a scalar subquery with two rows is an error on Postgres and its first row
//! on SQLite, never two outer rows.
//!
//! # Set-returning calls on Postgres
//!
//! In a select list, Postgres multiplies a row only by evaluating a set-returning function or
//! operator, and the parser only produces one from a call or an operator spelled in the
//! statement: a cast function must not return a set (`CREATE CAST` refuses one), and `CASE`,
//! `COALESCE`, `IS TRUE`, `IN (…)`, `WHERE`, `HAVING`, `LIMIT` and a window definition refuse
//! one outright. But one nested anywhere else in an expression at this level still multiplies the
//! row — under a function (`abs(generate_series(-1, 1))`), a cast, `IS NOT NULL`, `IS DISTINCT
//! FROM`, `NULLIF`'s argument — and so does one in `ORDER BY` or `DISTINCT ON`, which the planner
//! evaluates with the select list. `CREATE OPERATOR` over a `SETOF` function makes an operator
//! that multiplies rows exactly as the function does.
//!
//! So the proof reads every function name and every operator name at this level — the select
//! list, `ORDER BY`, `DISTINCT ON`, window definitions, and each subquery's tested expression
//! (`x IN (SELECT …)`), but not a subquery's own statement — and looks each up in the catalog by
//! name alone, across every overload and schema ([`Catalog`]). The resolver can only pick an
//! overload of a name it was given, so reading all of them is a superset of whatever it picks. A
//! name the catalog does not have is unproven too.
//!
//! SQLite has no set-returning function a select list may call — `json_each` and
//! `generate_series` are table-valued, live in `FROM`, and are "no such function" in a select
//! list — and no way to define an operator, so there the rule is empty.
//!
//! SQLite statements are read with libpg_query, as `infer` and `jsonshape` read them, after their
//! `?n` placeholders become `$n`. Clause structure is the SQL both dialects share, and SQLite's
//! own syntax — `LIMIT 5, 0`, `[bracketed]` names — does not parse there, which proves nothing.

use crate::dialect::Dialect;
use crate::infer::{self, Trust};
use crate::jsonshape::last_string;
use crate::pgtypes::oid;
use pg_query::protobuf::{self, node::Node as NodeEnum, SelectStmt};
use serde_json::Value;
use std::collections::HashSet;

/// The shapes the build proves return exactly one row, for the diagnostic that refuses a
/// middleware it cannot prove.
pub const PROVABLE: &str = "a SELECT with no FROM and no WHERE, such as `SELECT EXISTS (SELECT 1 FROM org_members WHERE org_id = :org_id AND user_id = :auth.user_id) AS authorized` or `SELECT (SELECT role FROM org_members WHERE org_id = :org_id AND user_id = :auth.user_id) AS role`; or a SELECT whose select list calls an aggregate (count, sum, avg, min, max) outside a subquery and without OVER, such as `SELECT count(*) > 0 AS authorized FROM org_members WHERE org_id = :org_id AND user_id = :auth.user_id`. Either may start with a WITH. Neither may have GROUP BY, HAVING, LIMIT, OFFSET, UNION, INTERSECT, EXCEPT, or VALUES, or call a set-returning function such as generate_series or unnest outside a subquery";

/// The catalog facts the proof needs and the parse tree cannot carry, all by name. The host
/// answers the three queries below and the orchestration fills this in.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    /// Function names with at least one entry in `pg_proc`.
    pub functions: HashSet<String>,
    /// Function names with at least one set-returning overload (`proretset`), in any schema.
    pub set_returning: HashSet<String>,
    /// Operator names some set-returning function implements, in any schema.
    pub set_returning_operators: HashSet<String>,
    /// Per relation, the column lists of its unique keys: every unique, valid, non-partial index
    /// over plain columns. This is the fact [`at_most_one`] rests on, and it is a fact about the
    /// database rather than about the statement — adding an index makes a read provably single
    /// and dropping one makes it plural again, which is the inference doing its job.
    pub unique_keys: std::collections::HashMap<String, Vec<Vec<String>>>,
    /// Per relation, the unique keys that hold only among the rows a predicate admits (a partial
    /// unique index, `… (email) WHERE deleted_at IS NULL`), with the predicate as the server
    /// prints it. One bounds a read only where the WHERE restricts the relation to such rows by
    /// conjuncts of its own ([`From`]).
    pub partial_keys: std::collections::HashMap<String, Vec<(Vec<String>, String)>>,
    /// Per relation as named in `unique_keys`, the type oid of each of its columns (a domain's by
    /// its base type). A key column is equated by a value of a type its index's operator family
    /// compares without converting the column (see `equality_bounds`), and a column on the other
    /// side of the equality has the type given here; a relation missing here was read by a host
    /// that does not report types, and its keys take any value, as before.
    pub column_types: std::collections::HashMap<String, std::collections::HashMap<String, u32>>,
    /// Per relation as named in `unique_keys`, its columns under a nondeterministic collation,
    /// whose equality is not identity: `'a' = 'A'` there, and a function of the column can tell
    /// the two apart.
    pub nondeterministic: std::collections::HashMap<String, HashSet<String>>,
    /// Function names with a volatile overload (`provolatile = 'v'`). A volatile call is evaluated
    /// for each row, so `id = random_id()` is no value a key can be pinned to.
    pub volatile: HashSet<String>,
    /// Operator names a user or an extension defined, as the trust check reports them: such an
    /// operator may be the one a comparison or an arithmetic resolves to, and may be volatile.
    pub user_operators: HashSet<String>,
    /// The built-in type names no user's cast function targets, as the trust check reports them,
    /// or `None` where the host did not ask. A cast to another type may call such a function,
    /// which may be volatile.
    pub cast_targets: Option<HashSet<String>>,
    /// Every alias the whole statement gives a relation, to its name as `unique_keys` keys it, for
    /// a subquery proven on its own whose equality reads a column of an enclosing query
    /// (`u.id = p.author_id`): that column's type is the enclosing relation's. An alias the
    /// statement gives two relations is absent.
    pub outer_aliases: std::collections::HashMap<String, String>,
    /// The statement's parameter types in `$n` order, from its describe. Empty when the host did
    /// not describe it, and then an uncast parameter is taken to have the column's type, which is
    /// what the server infers for `col = $1`.
    pub param_types: Vec<u32>,
    /// Whether a user or an extension defined an operator named `=`, which may then be the one a
    /// comparison of other types than the key column's resolves to.
    pub equality_shadowed: bool,
    /// Per relation, the events (`TRIGGER_ROW_INSERT`, `_UPDATE`, `_DELETE` as bits) for which it,
    /// or a partition or an inheritance child of it, has an enabled row trigger that runs BEFORE
    /// the change or INSTEAD of it. Such a trigger can return NULL and skip the row, so an INSERT
    /// of one row returns at most one.
    pub skipping_triggers: std::collections::HashMap<String, i32>,
    /// Per relation, the events (the same bits) a rewrite rule replaces or adds actions to, and
    /// for a view every event it has no INSTEAD OF trigger for, as it hands such a write to a table
    /// whose triggers and rules this catalog did not read. RETURNING then gives any number of rows.
    pub rewritten: std::collections::HashMap<String, i32>,
    /// The names of the WITH queries of the statements enclosing the one being proven, which a
    /// subquery, a WITH query's body or a set operation's arm is proven on its own as: such a
    /// name read there unqualified is that WITH query, never the table of the same name.
    pub enclosing_ctes: Vec<String>,
}

impl Catalog {
    /// This catalog for a statement nested in one whose WITH queries are `names`.
    fn enclosed(&self, names: &[String]) -> Catalog {
        let mut inner = self.clone();
        inner.enclosing_ctes.extend(names.iter().cloned());
        inner
    }
}

/// The names of a SELECT's own WITH queries.
fn with_names(s: &SelectStmt) -> Vec<String> {
    s.with_clause.iter().flat_map(|w| w.ctes.iter().filter_map(cte_name).cloned()).collect()
}

/// The `pg_class` row of the relation named in the text column `$name`, as the server resolves the
/// name the statement writes: through the search path when unqualified, and each part quoted, so
/// a name that needs quotes (`"Users"`) resolves as written.
macro_rules! relation_named {
    ($name:literal) => {
        concat!(
            "pg_catalog.to_regclass(CASE WHEN strpos(", $name, ", '.') > 0 \
      THEN pg_catalog.quote_ident(split_part(", $name, ", '.', 1)) || '.' || pg_catalog.quote_ident(split_part(", $name, ", '.', 2)) \
      ELSE pg_catalog.quote_ident(", $name, ") END)"
        )
    };
}

/// The `pg_trigger.tgtype` bits [`Catalog::skipping_triggers`] reports.
pub const TRIGGER_ROW_INSERT: i32 = 4;
pub const TRIGGER_ROW_DELETE: i32 = 8;
pub const TRIGGER_ROW_UPDATE: i32 = 16;

/// `$1` is `text[]` of relation names as the statement writes them: one row per relation a write
/// to which may return no row or many, `(the name, the events that may skip the row, the events a
/// rule rewrites)`, each as tgtype bits. An event may skip the row where the relation, or a
/// partition or an inheritance child a row is routed to, has an enabled row-level trigger that
/// runs BEFORE the change or INSTEAD of it (internal triggers, the foreign keys', run AFTER and
/// skip nothing), or is a foreign table, whose wrapper may write nothing. A rule (`pg_rewrite`) on
/// the relation rewrites its event, and so does a view every event it has no INSTEAD OF trigger
/// for, since a write through it reaches a table whose triggers and rules these rows do not cover;
/// an INSTEAD OF trigger returns its own row or none.
pub const TRIGGERS_QUERY: &str = concat!(
    "WITH RECURSIVE rels AS (SELECT c.oid AS root, c.relkind, x.name FROM unnest($1::text[]) AS x(name) JOIN pg_class c ON c.oid = ",
    relation_named!("x.name"),
    "), \
    tree(root, rel) AS (SELECT root, root FROM rels UNION SELECT t.root, i.inhrelid FROM tree t JOIN pg_inherits i ON i.inhparent = t.rel), \
    found AS (SELECT r.name, \
      (coalesce((SELECT bit_or(g.tgtype::int4 & 28) FROM tree t JOIN pg_trigger g ON g.tgrelid = t.rel \
          WHERE t.root = r.root AND NOT g.tgisinternal AND g.tgenabled <> 'D' AND g.tgtype::int4 & 1 = 1 AND g.tgtype::int4 & 66 <> 0), 0) \
        | CASE WHEN EXISTS (SELECT 1 FROM tree t JOIN pg_class f ON f.oid = t.rel WHERE t.root = r.root AND f.relkind = 'f') THEN 28 ELSE 0 END)::int4 AS skipping, \
      (CASE WHEN r.relkind = 'v' THEN 28 & ~coalesce((SELECT bit_or(g.tgtype::int4 & 28) FROM pg_trigger g \
          WHERE g.tgrelid = r.root AND g.tgenabled <> 'D' AND g.tgtype::int4 & 65 = 65), 0) ELSE 0 END \
        | coalesce((SELECT bit_or(CASE w.ev_type WHEN '2' THEN 16 WHEN '3' THEN 4 ELSE 8 END) FROM pg_rewrite w \
          WHERE w.ev_class = r.root AND w.ev_type IN ('2', '3', '4') AND w.ev_enabled <> 'D'), 0))::int4 AS rewritten \
      FROM rels r) \
    SELECT name, skipping, rewritten FROM found WHERE skipping <> 0 OR rewritten <> 0"
);

/// `$1` is `text[]` of relation names as the statement writes them (`users`, `archive.users`):
/// one row per unique index that bounds what a scan of that relation reads, `(the name, key
/// parts, the type oid of each part that is an expression, the predicate of a partial index)`. A
/// part is a column's name, or an expression as the server prints it, in parentheses:
/// `(lower(email))`. The planner's own conditions (`relation_has_unique_index_ext`): the index is
/// valid and immediate (a DEFERRABLE constraint lets duplicates stand inside a transaction that
/// defers it); the relation is not a traditional inheritance parent, whose scan also reads
/// children the index does not cover (a partitioned table's unique index holds across its
/// partitions); and each part's index collation agrees with the column's on equality (the same
/// collation, or both deterministic), since a comparison under the column's collation is what the
/// WHERE makes. An expression part takes the collation of what it reads, which SQL cannot read
/// back, so it counts only where that is certainly deterministic: an expression of a type with no
/// collation, or one with no COLLATE of its own over columns of deterministic collations only (a
/// deterministic equality is identity, which every collation's equality holds to). A partial
/// index's predicate is the proof's to check. A name resolves as the server resolves it, through
/// the search path when unqualified.
pub const UNIQUE_KEYS_QUERY: &str = concat!(
    "SELECT x.name, \
      array_agg(CASE WHEN k.attnum = 0 THEN '(' || pg_catalog.pg_get_indexdef(i.indexrelid, k.ord::int, true) || ')' ELSE a.attname::text END ORDER BY k.ord), \
      array_agg(CASE WHEN k.attnum = 0 THEN a.atttypid::int8 END ORDER BY k.ord), \
      min(pg_catalog.pg_get_expr(i.indpred, i.indrelid, true)) \
    FROM unnest($1::text[]) AS x(name) \
    JOIN pg_class c ON c.oid = ",
    relation_named!("x.name"),
    " JOIN pg_index i ON i.indrelid = c.oid \
    JOIN unnest(i.indkey::int2[], i.indcollation::oid[]) WITH ORDINALITY AS k(attnum, coll, ord) ON k.ord <= i.indnkeyatts \
    JOIN pg_attribute a ON CASE WHEN k.attnum = 0 THEN a.attrelid = i.indexrelid AND a.attnum = k.ord \
      ELSE a.attrelid = i.indrelid AND a.attnum = k.attnum AND NOT a.attisdropped END \
    WHERE i.indisunique AND i.indisvalid AND i.indimmediate \
      AND (c.relkind = 'p' OR NOT EXISTS (SELECT 1 FROM pg_inherits h WHERE h.inhparent = c.oid)) \
    GROUP BY x.name, i.indexrelid, i.indnkeyatts \
    HAVING count(*) = i.indnkeyatts AND bool_and(CASE WHEN k.attnum = 0 THEN k.coll = 0 OR ( \
        pg_catalog.pg_get_indexdef(i.indexrelid, k.ord::int, true) !~* 'collate' \
        AND NOT EXISTS (SELECT 1 FROM pg_depend d JOIN pg_attribute r ON r.attrelid = d.refobjid AND r.attnum = d.refobjsubid \
          JOIN pg_collation o ON o.oid = r.attcollation \
          WHERE d.classid = 'pg_class'::regclass AND d.objid = i.indexrelid AND d.refclassid = 'pg_class'::regclass \
            AND d.refobjsubid > 0 AND NOT o.collisdeterministic)) \
      ELSE k.coll = a.attcollation OR ( \
        coalesce((SELECT o.collisdeterministic FROM pg_collation o WHERE o.oid = k.coll), true) \
        AND coalesce((SELECT o.collisdeterministic FROM pg_collation o WHERE o.oid = a.attcollation), true)) END)"
);

/// `$1` is `text[]` of relation names as the statement writes them: every column of each, `(the
/// name, column names, their type oids, whether each one's collation is deterministic)`, a
/// domain's column by its base type, whose equality it uses. What the other side of an equality
/// is when it is a column (`u.id = p.author_id`), which relation a bare column belongs to, and
/// whether a column's equality is identity.
pub const COLUMNS_QUERY: &str = concat!(
    "SELECT x.name, array_agg(a.attname::text ORDER BY a.attnum), \
      array_agg((CASE WHEN t.typtype = 'd' THEN t.typbasetype ELSE a.atttypid END)::int8 ORDER BY a.attnum), \
      array_agg(coalesce((SELECT o.collisdeterministic FROM pg_collation o WHERE o.oid = a.attcollation), true) ORDER BY a.attnum) \
    FROM unnest($1::text[]) AS x(name) \
    JOIN pg_class c ON c.oid = ",
    relation_named!("x.name"),
    " JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped \
    JOIN pg_type t ON t.oid = a.atttypid \
    GROUP BY x.name"
);

/// `$1` is `text[]` of function names: `(proname, has a set-returning overload, has a volatile
/// overload)`.
pub const FUNCTIONS_QUERY: &str =
    "SELECT proname::text, bool_or(proretset), bool_or(provolatile = 'v') FROM pg_proc WHERE proname::text = ANY($1::text[]) GROUP BY proname";

/// Every operator some set-returning function implements.
pub const SET_RETURNING_OPERATORS_QUERY: &str =
    "SELECT DISTINCT o.oprname::text FROM pg_operator o JOIN pg_proc p ON p.oid = o.oprcode WHERE p.proretset";

/// Prove that one rewritten statement (placeholders already `$n` or `?n`) returns exactly one
/// row, or say why the build cannot, worded to finish a sentence: "a WHERE with no FROM can
/// remove the row". `trust` is the describer's ([`crate::describe::Describer::trust`]); `catalog`
/// is read on Postgres only.
pub fn exactly_one(sql: &str, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    let s = one_select(sql, dialect)?;
    exactly_one_select(&s, dialect, trust, catalog)
}

fn exactly_one_select(s: &SelectStmt, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    if !infer::single_row(s) {
        return Err(not_one_row_shaped(s).to_string());
    }
    if s.into_clause.is_some() {
        return Err("SELECT INTO creates a table and returns no rows".into());
    }
    if dialect == Dialect::Postgres {
        let at_this_level = s.target_list.iter().chain(&s.sort_clause).chain(&s.distinct_clause).chain(&s.window_clause);
        if let Some(why) = calls_at_this_level(at_this_level).set_returning(catalog) {
            return Err(why);
        }
    }
    let aggregates = s.target_list.iter().any(|target| match target.node.as_ref() {
        Some(NodeEnum::ResTarget(rt)) => rt.val.as_deref().is_some_and(|val| infer::aggregates(val, dialect, trust)),
        _ => false,
    });
    if aggregates {
        return Ok(());
    }
    if let Some(through) = cte_passthrough(s, dialect, trust, catalog) {
        return match through {
            Ok(Cardinality::One) if s.where_clause.is_none() => Ok(()),
            Ok(_) => Err("a WHERE over the WITH query can remove its row".into()),
            Err(why) => Err(why),
        };
    }
    if !s.from_clause.is_empty() {
        return Err("a FROM returns a row per row it matches, and only an aggregate with no GROUP BY collapses them to one".into());
    }
    if s.where_clause.is_some() {
        return Err("a WHERE with no FROM can remove the row".into());
    }
    Ok(())
}

/// Prove that one rewritten statement returns **at most** one row — no row, or one — or say why
/// the build cannot, worded to finish a sentence the same way [`exactly_one`]'s does.
///
/// This is the weaker reading, and it is the one a nested `SUBQ` takes its shape from: at most one
/// row is an object or null, and anything else is a list. Three things prove it, in the order the
/// build tries them:
///
/// 1. A constant `LIMIT 0` or `LIMIT 1`, which caps the count whatever the rest of the statement
///    does — a set operation and a set-returning call included, because the limit applies last.
/// 2. Exactly one row, which is at most one row. That covers `SELECT count(*) …` and the shapes
///    with no `FROM` at all.
/// 3. A unique key of the one relation the statement reads, every column of it equated to
///    something that is not a column of that relation. Two rows cannot share a unique key, and an
///    equality never matches NULL, so at most one row survives — which holds whether or not the
///    key's columns are `NOT NULL`, since a unique index only lets NULLs repeat.
///
/// The third is why an index changes an inferred type. The proof reads the index that exists at
/// build time; `reylo.lock` and the boot re-describe turn a later `CREATE`/`DROP INDEX` into a
/// contract mismatch the deploy reports, rather than a shape that quietly drifts.
pub fn at_most_one(sql: &str, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    let s = one_select(sql, dialect)?;
    at_most_one_select(&s, dialect, trust, catalog)
}

fn at_most_one_select(s: &SelectStmt, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    if let Some(n) = const_limit(s) {
        if n <= 1 {
            return Ok(());
        }
    }
    if exactly_one_select(s, dialect, trust, catalog).is_ok() {
        return Ok(());
    }
    if s.limit_count.is_some() {
        // Anything still here is a LIMIT above one, or one written as an expression the build
        // cannot read. An OFFSET alone only ever removes rows, so it is left to the key proof.
        return Err("a LIMIT of more than one row, or one the build cannot read, bounds nothing".into());
    }
    // An INTERSECT returns only rows both arms return, and an EXCEPT only rows of its left arm,
    // with ALL or without: at most one row when an arm it draws every row from is.
    if let (Some(left), Some(right)) = (s.larg.as_deref(), s.rarg.as_deref()) {
        let inner = catalog.enclosed(&with_names(s));
        let arm = |a: &SelectStmt| at_most_one_select(a, dialect, trust, &inner).is_ok();
        if s.op == protobuf::SetOperation::SetopIntersect as i32 && (arm(left) || arm(right)) {
            return Ok(());
        }
        if s.op == protobuf::SetOperation::SetopExcept as i32 && arm(left) {
            return Ok(());
        }
    }
    let grouped = !s.group_clause.is_empty() && s.larg.is_none() && s.rarg.is_none() && s.values_lists.is_empty() && s.into_clause.is_none();
    if let Some(why) = not_key_shaped(s) {
        // A GROUP BY has its own proof below; every other shape ends here.
        if !grouped {
            return Err(why.to_string());
        }
    }
    if dialect == Dialect::Postgres {
        let at_this_level = s.target_list.iter().chain(&s.sort_clause).chain(&s.distinct_clause).chain(&s.window_clause);
        if let Some(why) = calls_at_this_level(at_this_level).set_returning(catalog) {
            return Err(why);
        }
    }
    if grouped {
        return one_group(s, dialect, trust, catalog);
    }
    if let Some(through) = cte_passthrough(s, dialect, trust, catalog) {
        return through.map(|_| ());
    }
    one_row(s, dialect, trust, catalog)
}

/// `WITH x AS (…) SELECT … FROM x [WHERE …]`: the rows are the WITH query's, at most. The WITH
/// query is an INSERT, UPDATE or DELETE with RETURNING, proven by shape, or a SELECT, proven
/// like any other. `None` when the statement is not that shape.
fn cte_passthrough(s: &SelectStmt, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Option<Result<Cardinality, String>> {
    let [only] = s.from_clause.as_slice() else { return None };
    let Some(NodeEnum::RangeVar(rv)) = only.node.as_ref() else { return None };
    // A schema-qualified name is the table, never the WITH query of that name.
    if !rv.schemaname.is_empty() {
        return None;
    }
    let with = s.with_clause.as_ref()?;
    let cte = with.ctes.iter().find(|c| cte_name(c) == Some(&rv.relname))?;
    let Some(NodeEnum::CommonTableExpr(cte)) = cte.node.as_ref() else { return None };
    if with.recursive || !s.group_clause.is_empty() || s.having_clause.is_some() || s.larg.is_some() || s.rarg.is_some() || !s.distinct_clause.is_empty() {
        return None;
    }
    if dialect == Dialect::Postgres {
        let at_this_level = s.target_list.iter().chain(&s.sort_clause).chain(&s.distinct_clause).chain(&s.window_clause);
        if let Some(why) = calls_at_this_level(at_this_level).set_returning(catalog) {
            return Some(Err(why));
        }
    }
    let body = cte.ctequery.as_deref()?.node.as_ref()?;
    let enclosed = catalog.enclosed(&with_names(s));
    let inner = match body {
        NodeEnum::SelectStmt(inner) => match exactly_one_select(inner, dialect, trust, &enclosed) {
            Ok(()) => Ok(Cardinality::One),
            Err(why) => match at_most_one_select(inner, dialect, trust, &enclosed) {
                Ok(()) => Ok(Cardinality::AtMostOne),
                Err(_) => Err(format!("the WITH query `{}` returns any number of rows: {why}", rv.relname)),
            },
        },
        NodeEnum::InsertStmt(_) | NodeEnum::UpdateStmt(_) | NodeEnum::DeleteStmt(_) => match dml_node(body, catalog) {
            Some(r) => r.map_err(|why| format!("the WITH query `{}` returns any number of rows: {why}", rv.relname)),
            None => return None,
        },
        _ => return None,
    };
    Some(match inner {
        Ok(Cardinality::One) if s.where_clause.is_some() => Ok(Cardinality::AtMostOne),
        other => other,
    })
}

/// The grouped form of the proof: one group at most when every input row carries the same value
/// of every `GROUP BY` expression ([`From::pinned`]), or when every FROM item is determined and so
/// there is one input row at most. What else is joined does not matter: it can multiply the input
/// rows, never the groups. `HAVING` only drops the group. This is `SELECT u.*, count(p.id) FROM
/// users u LEFT JOIN posts p ON … WHERE u.id = $1 GROUP BY u.id`, and `SELECT author_id, count(*)
/// FROM posts WHERE author_id = $1 GROUP BY author_id`.
fn one_group(s: &SelectStmt, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    const WHY: &str = "GROUP BY returns one row per group; the engine proves one group only when every GROUP BY expression is pinned to one value: a column of a relation whose unique key the WHERE pins, or a column the WHERE equates to a value";
    if s.group_clause.iter().any(|g| matches!(g.node.as_ref(), Some(NodeEnum::GroupingSet(_)))) {
        return Err("GROUPING SETS, ROLLUP and CUBE return a row per grouping set".into());
    }
    let from = From::read(s, catalog);
    if from.refused.is_some() {
        return Err(WHY.into());
    }
    if from.determined(false, dialect, trust, catalog).iter().all(|d| *d) {
        return Ok(());
    }
    let constant = from.determined(true, dialect, trust, catalog);
    let pinned = |g: &protobuf::Node| match g.node.as_ref() {
        // `GROUP BY 2` groups by the second output column.
        Some(NodeEnum::AConst(c)) => match &c.val {
            Some(protobuf::a_const::Val::Ival(i)) => output(s, i.ival).is_some_and(|e| from.pinned(e, &constant, catalog)),
            _ => false,
        },
        _ => from.pinned(g, &constant, catalog),
    };
    if s.group_clause.iter().all(pinned) {
        Ok(())
    } else {
        Err(WHY.into())
    }
}

/// The expression of output column `n`, counting from one.
fn output(s: &SelectStmt, n: i32) -> Option<&protobuf::Node> {
    match s.target_list.get(usize::try_from(n).ok()?.checked_sub(1)?)?.node.as_ref()? {
        NodeEnum::ResTarget(r) => r.val.as_deref(),
        _ => None,
    }
}

/// One SELECT's FROM, flattened out of its joins: the items, and the equalities that may pin a
/// unique key of one of them.
///
/// Call an item **determined** when every row of the FROM carries the same row of it, or that
/// row's null extension. When every item is, the FROM has one row at most: a join of items of one
/// row each is one row, and so is a LEFT JOIN of one row to an item determined against it. The
/// closure that finds them (the planner's `innerrel_is_unique` and Calcite's unique keys, in
/// composition):
///
/// 1. A WHERE conjunct `r.k = v` for each column of a unique key of relation `r`, where `v` reads
///    no column of `r` and only columns of determined items (or values), determines `r`, in any
///    join: the `=` rejects a null-extended row.
/// 2. The same equalities in the ON clause of an inner join determine either side, and of a LEFT
///    JOIN its right side (RIGHT JOIN mirrors it): each row of the other side meets one row of the
///    item at most, or its null extension. A FULL JOIN proves nothing, since it returns a row for
///    each side that matches nothing.
/// 3. A WITH query or a subquery the proof shows to be at most one row is determined; a LATERAL
///    one once every item before it is, because it is at most one row per row of those.
/// 4. A function in FROM never is.
///
/// Each equality must also be the key's own equality ([`equality_bounds`]), and `v` must call
/// nothing volatile, which is evaluated anew for each row.
struct From<'a> {
    items: Vec<Item<'a>>,
    conjuncts: Vec<Conjunct<'a>>,
    /// The names of the statement's WITH queries, which a subquery's FROM may read too.
    ctes: Vec<String>,
    /// The column names a USING join merges, and whether a NATURAL join merges any: a bare such
    /// name reads the merged column, which is of a type common to both sides and neither one's.
    using: Vec<String>,
    natural: bool,
    /// Why no closure holds over this FROM at all.
    refused: Option<&'static str>,
}

/// One FROM item.
struct Item<'a> {
    /// The name its columns go by: the alias, or the relation's own name.
    alias: String,
    source: Source<'a>,
    /// Whether an outer join can null-extend it: the right side of a LEFT JOIN, the left of a
    /// RIGHT JOIN, inside either.
    nullable: bool,
    /// Whether its alias renames its columns (`users u (a, b)`), so a column's name is not the
    /// catalog's.
    renamed: bool,
}

enum Source<'a> {
    /// A table or a view.
    Relation(&'a protobuf::RangeVar),
    /// A WITH query read by name, and whether the WITH is RECURSIVE.
    With(&'a protobuf::CommonTableExpr, bool),
    /// A subquery, and whether it is LATERAL.
    Subquery(&'a SelectStmt, bool),
    /// A WITH query of an enclosing statement, read by name: the proof does not read its body.
    Enclosing,
    /// A function, `TABLESAMPLE`, `XMLTABLE`, `JSON_TABLE`: nothing the proof reads.
    Opaque,
}

/// A conjunct of the WHERE or of an ON clause.
struct Conjunct<'a> {
    node: &'a protobuf::Node,
    /// For an ON clause, the items it may pin, by index: either side of an inner join, the right
    /// side of a LEFT JOIN, the left of a RIGHT JOIN. `None` for the WHERE, which may pin any.
    may_pin: Option<Vec<usize>>,
    /// Whether it holds of every row the FROM returns: the WHERE's, and an inner join's ON.
    inner: bool,
}

/// The conjuncts of a condition's top-level `AND`.
fn conjuncts_of(node: &protobuf::Node) -> Vec<&protobuf::Node> {
    match node.node.as_ref() {
        Some(NodeEnum::BoolExpr(b)) if b.boolop == protobuf::BoolExprType::AndExpr as i32 => b.args.iter().flat_map(conjuncts_of).collect(),
        _ => vec![node],
    }
}

impl<'a> From<'a> {
    fn read(s: &'a SelectStmt, catalog: &Catalog) -> From<'a> {
        let mut ctes = with_names(s);
        ctes.extend(catalog.enclosing_ctes.iter().cloned());
        let mut from = From { items: Vec::new(), conjuncts: Vec::new(), ctes, using: Vec::new(), natural: false, refused: None };
        for item in &s.from_clause {
            from.flatten(item, s.with_clause.as_ref(), false);
        }
        if let Some(w) = s.where_clause.as_deref() {
            for node in conjuncts_of(w) {
                from.conjuncts.push(Conjunct { node, may_pin: None, inner: true });
            }
        }
        from
    }

    /// One relation alone, as an UPDATE or a DELETE reads its target, with the WHERE's conjuncts.
    fn single(rv: &'a protobuf::RangeVar, where_clause: Option<&'a protobuf::Node>, with: Option<&'a protobuf::WithClause>) -> From<'a> {
        let ctes = with.iter().flat_map(|w| w.ctes.iter().filter_map(cte_name).cloned()).collect();
        let (alias, renamed) = rv.alias.as_ref().map_or_else(|| (rv.relname.clone(), false), |a| (a.aliasname.clone(), !a.colnames.is_empty()));
        let item = Item { alias, source: Source::Relation(rv), nullable: false, renamed };
        let conjuncts = where_clause.into_iter().flat_map(conjuncts_of).map(|node| Conjunct { node, may_pin: None, inner: true }).collect();
        From { items: vec![item], conjuncts, ctes, using: Vec::new(), natural: false, refused: None }
    }

    fn flatten(&mut self, node: &'a protobuf::Node, with: Option<&'a protobuf::WithClause>, nullable: bool) {
        let named = |a: &protobuf::Alias| (a.aliasname.clone(), !a.colnames.is_empty());
        match node.node.as_ref() {
            Some(NodeEnum::RangeVar(rv)) => {
                let (alias, renamed) = rv.alias.as_ref().map(named).unwrap_or_else(|| (rv.relname.clone(), false));
                let cte = with.filter(|_| rv.schemaname.is_empty()).and_then(|w| {
                    w.ctes.iter().find_map(|c| match c.node.as_ref() {
                        Some(NodeEnum::CommonTableExpr(c)) if c.ctename == rv.relname => Some((c, w.recursive)),
                        _ => None,
                    })
                });
                let source = match cte {
                    Some((c, recursive)) => Source::With(c, recursive),
                    None if rv.schemaname.is_empty() && self.ctes.contains(&rv.relname) => Source::Enclosing,
                    None => Source::Relation(rv),
                };
                self.items.push(Item { alias, source, nullable, renamed });
            }
            Some(NodeEnum::RangeSubselect(r)) => {
                let (alias, renamed) = r.alias.as_ref().map(named).unwrap_or_default();
                let source = match r.subquery.as_deref().and_then(|n| n.node.as_ref()) {
                    Some(NodeEnum::SelectStmt(sub)) => Source::Subquery(sub, r.lateral),
                    _ => Source::Opaque,
                };
                self.items.push(Item { alias, source, nullable, renamed });
            }
            Some(NodeEnum::JoinExpr(j)) => {
                use protobuf::JoinType;
                if j.alias.is_some() {
                    self.refused = Some("a join with an alias of its own hides the names of the relations it joins");
                }
                let kind = j.jointype;
                self.natural |= j.is_natural;
                self.using.extend(j.using_clause.iter().filter_map(|n| field_name(n).map(String::from)));
                let (left, right) = if kind == JoinType::JoinLeft as i32 {
                    (nullable, true)
                } else if kind == JoinType::JoinRight as i32 {
                    (true, nullable)
                } else if kind == JoinType::JoinInner as i32 {
                    (nullable, nullable)
                } else {
                    self.refused = Some("a FULL JOIN returns a row for each side that matches nothing, so two rows where each side has one");
                    (true, true)
                };
                let start = self.items.len();
                if let Some(l) = j.larg.as_deref() {
                    self.flatten(l, with, left);
                }
                let middle = self.items.len();
                if let Some(r) = j.rarg.as_deref() {
                    self.flatten(r, with, right);
                }
                let end = self.items.len();
                // USING and NATURAL equate columns the parse tree does not spell out, so they
                // pin nothing here; the WHERE still may.
                if let (Some(quals), true, false) = (j.quals.as_deref(), j.using_clause.is_empty(), j.is_natural) {
                    let (may_pin, inner): (Vec<usize>, bool) = if kind == JoinType::JoinInner as i32 {
                        ((start..end).collect(), true)
                    } else if kind == JoinType::JoinLeft as i32 {
                        ((middle..end).collect(), false)
                    } else if kind == JoinType::JoinRight as i32 {
                        ((start..middle).collect(), false)
                    } else {
                        (Vec::new(), false)
                    };
                    for node in conjuncts_of(quals) {
                        self.conjuncts.push(Conjunct { node, may_pin: Some(may_pin.clone()), inner });
                    }
                }
            }
            other => {
                // A function goes by its alias, or by its own name; anything else without an alias
                // could be read under a name the proof does not know.
                let alias = match other {
                    Some(NodeEnum::RangeFunction(f)) => f.alias.as_ref().map(|a| a.aliasname.clone()).or_else(|| {
                        let tree = serde_json::to_value(&f.functions).ok()?;
                        first_function_name(&tree)
                    }),
                    Some(NodeEnum::RangeTableSample(t)) => match t.relation.as_deref().and_then(|r| r.node.as_ref()) {
                        Some(NodeEnum::RangeVar(rv)) => Some(rv.alias.as_ref().map_or_else(|| rv.relname.clone(), |a| a.aliasname.clone())),
                        _ => None,
                    },
                    Some(NodeEnum::RangeTableFunc(f)) => f.alias.as_ref().map(|a| a.aliasname.clone()),
                    _ => None,
                };
                match alias {
                    Some(alias) => self.items.push(Item { alias, source: Source::Opaque, nullable, renamed: true }),
                    None => self.refused = Some("a FROM item the engine cannot name"),
                }
            }
        }
    }

    /// Which items are determined, by index: the closure in the type's docs. With `constant`,
    /// only items no row null-extends count, pinned by the WHERE or by inner joins: every row of
    /// the FROM then carries the same row of each, which is what grouping and DISTINCT ask.
    fn determined(&self, constant: bool, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Vec<bool> {
        let n = self.items.len();
        let mut done = vec![false; n];
        loop {
            let mut changed = false;
            for i in 0..n {
                if done[i] {
                    continue;
                }
                let item = &self.items[i];
                let pinned = match &item.source {
                    Source::Relation(rv) => self.key_pinned(i, rv, &done, constant, catalog),
                    _ if constant && item.nullable => false,
                    Source::With(cte, recursive) => !*recursive && with_query_at_most_one(cte, dialect, trust, &catalog.enclosed(&self.ctes)),
                    Source::Subquery(sub, lateral) => (!*lateral || (0..n).all(|j| j == i || done[j])) && at_most_one_select(sub, dialect, trust, &catalog.enclosed(&self.ctes)).is_ok(),
                    Source::Enclosing | Source::Opaque => false,
                };
                if pinned {
                    done[i] = true;
                    changed = true;
                }
            }
            if !changed {
                return done;
            }
        }
    }

    /// Whether the equalities pin every column of a unique key of item `i`.
    fn key_pinned(&self, i: usize, rv: &protobuf::RangeVar, done: &[bool], constant: bool, catalog: &Catalog) -> bool {
        let item = &self.items[i];
        if item.renamed {
            return false;
        }
        let relation = catalog_name(rv);
        let usable = |c: &Conjunct| match &c.may_pin {
            None => true,
            Some(may) => may.contains(&i) && (!constant || (c.inner && !item.nullable)),
        };
        let full = catalog.unique_keys.get(&relation).into_iter().flatten().map(|key| (key, None));
        let partial = catalog.partial_keys.get(&relation).into_iter().flatten().map(|(key, predicate)| (key, Some(predicate.as_str())));
        full.chain(partial).any(|(key, predicate)| {
            !key.is_empty()
                && key.iter().all(|part| self.conjuncts.iter().any(|c| usable(c) && self.equates(c.node, i, &relation, part, done, catalog)))
                && predicate.is_none_or(|p| self.restricts(i, p))
        })
    }

    /// Whether the WHERE restricts item `i` to the rows a partial index's predicate admits: each
    /// conjunct of the predicate is one of the WHERE's own top-level conjuncts, read with `i`'s
    /// name, and `col IS NOT NULL` is also met by an equality on `col`, which no NULL passes. A
    /// conjunct of an ON clause does not count: the index must bound the relation's rows before
    /// any join, as the planner learned (bug #17975). A string literal the server prints with a
    /// cast to text (`status = 'active'::text`) matches the same literal written without one.
    fn restricts(&self, i: usize, predicate: &str) -> bool {
        // A bare name in the WHERE may be a merged column, which is neither side's.
        if self.natural || !self.using.is_empty() {
            return false;
        }
        let Ok(parsed) = pg_query::parse(&format!("SELECT 1 WHERE {predicate}")) else { return false };
        let Some(NodeEnum::SelectStmt(select)) = parsed.protobuf.stmts.first().and_then(|s| s.stmt.as_deref()).and_then(|n| n.node.as_ref()) else { return false };
        let Some(wanted) = select.where_clause.as_deref() else { return false };
        let alias = &self.items[i].alias;
        let have: Vec<Value> = self.conjuncts.iter().filter(|k| k.may_pin.is_none()).map(|k| normalized(k.node, alias)).collect();
        conjuncts_of(wanted).into_iter().all(|w| {
            let w = normalized(w, alias);
            have.contains(&w) || not_null_column(&w).is_some_and(|column| have.iter().any(|h| equates_column(h, &column)))
        })
    }

    /// Whether `node` is `column = value` for this column of item `i`, where the value reads no
    /// column of `i` and only columns of `done` items, columns the WHERE pins to a value
    /// ([`From::pinned_by_value`]: `u.id = p.author_id` beside `p.author_id = $1`), or columns of
    /// enclosing queries, and compares under the column's own equality. A bare name for the column is `i`'s: `i` has it, so had
    /// another item it too, the server would have refused the name as ambiguous.
    fn equates(&self, node: &protobuf::Node, i: usize, relation: &str, column: &str, done: &[bool], catalog: &Catalog) -> bool {
        let Some(NodeEnum::AExpr(e)) = node.node.as_ref() else { return false };
        if !is_equality(e) {
            return false;
        }
        let (Some(l), Some(r)) = (e.lexpr.as_deref(), e.rexpr.as_deref()) else { return false };
        let alias = &self.items[i].alias;
        let is_part = |side: &protobuf::Node| match column.strip_prefix('(') {
            Some(_) => !self.mentions_merged(side) && matches_expression(side, column, alias),
            None => self.own(side, alias).as_deref() == Some(column),
        };
        [(l, r), (r, l)].into_iter().any(|(side, value)| {
            is_part(side)
                && self.reads(value, catalog).is_some_and(|read| {
                    read.iter().all(|(j, read)| *j != i && (done[*j] || read.as_deref().is_some_and(|c| self.pinned_by_value(*j, c, catalog))))
                })
                && equality_bounds(catalog, relation, column, value, &|n| self.node_type(n, catalog))
        })
    }

    /// Whether the WHERE equates this column of item `j` to a value that reads no column, so every
    /// row carries the same value of it: `p.author_id = $1`. The same under the column's own
    /// equality is one value only where that equality is identity ([`identity`]): `tag = 'a'`
    /// under a nondeterministic collation admits 'a' and 'A', which `ascii(tag)` tells apart.
    fn pinned_by_value(&self, j: usize, column: &str, catalog: &Catalog) -> bool {
        let item = &self.items[j];
        let Source::Relation(rv) = &item.source else { return false };
        if item.renamed {
            return false;
        }
        let relation = catalog_name(rv);
        if !identity(catalog, &relation, column) {
            return false;
        }
        self.conjuncts.iter().filter(|k| k.may_pin.is_none()).any(|k| {
            let Some(NodeEnum::AExpr(e)) = k.node.node.as_ref() else { return false };
            let (Some(l), Some(r)) = (e.lexpr.as_deref(), e.rexpr.as_deref()) else { return false };
            is_equality(e)
                && [(l, r), (r, l)].into_iter().any(|(side, value)| {
                    self.own(side, &item.alias).as_deref() == Some(column)
                        && self.reads(value, catalog).is_some_and(|read| read.is_empty())
                        && equality_bounds(catalog, &relation, column, value, &|n| self.node_type(n, catalog))
                })
        })
    }

    /// Whether every row of the FROM carries the same value of `expr`: a column of a `constant`
    /// item, a column the WHERE equates to a value under the column's own equality (the one
    /// grouping and DISTINCT use, so a pin through an explicit COLLATE does not count), or an
    /// expression that reads no column, calls nothing volatile and no window function, which
    /// reads other rows than its own (`row_number() OVER ()`).
    fn pinned(&self, expr: &protobuf::Node, constant: &[bool], catalog: &Catalog) -> bool {
        if mentions_window(expr) {
            return false;
        }
        if let Some(NodeEnum::ColumnRef(c)) = expr.node.as_ref() {
            let Some((i, column)) = self.column(c, catalog) else { return false };
            if constant[i] {
                return true;
            }
            let Source::Relation(rv) = &self.items[i].source else { return false };
            let relation = catalog_name(rv);
            return self.conjuncts.iter().any(|k| k.may_pin.is_none() && self.equates(k.node, i, &relation, &column, constant, catalog));
        }
        self.reads(expr, catalog).is_some_and(|read| read.is_empty()) && !calls_volatile(expr, catalog)
    }

    /// The column of the item read as `alias` this expression names ([`own_column`]), unless it is
    /// a bare name a USING or NATURAL join merges, which is neither side's column.
    fn own(&self, node: &protobuf::Node, alias: &str) -> Option<String> {
        let column = own_column(node, alias)?;
        let bare = matches!(node.node.as_ref(), Some(NodeEnum::ColumnRef(c)) if c.fields.len() == 1);
        (!bare || !self.merged(&column)).then_some(column)
    }

    /// Whether a bare name may read a column a USING or NATURAL join merges.
    fn merged(&self, column: &str) -> bool {
        self.natural || self.using.iter().any(|u| u == column)
    }

    /// Whether `node` reads a merged column by a bare name anywhere.
    fn mentions_merged(&self, node: &protobuf::Node) -> bool {
        if !self.natural && self.using.is_empty() {
            return false;
        }
        let Ok(tree) = serde_json::to_value(node) else { return true };
        fn walk(v: &Value, from: &From) -> bool {
            match v {
                Value::Object(map) => map.iter().any(|(key, child)| {
                    let bare = (key == "ColumnRef")
                        .then(|| child.get("fields")?.as_array().filter(|f| f.len() == 1)?.first()?.get("node")?.get("String")?.get("sval")?.as_str())
                        .flatten();
                    bare.is_some_and(|name| from.merged(name)) || walk(child, from)
                }),
                Value::Array(items) => items.iter().any(|i| walk(i, from)),
                _ => false,
            }
        }
        walk(&tree, self)
    }

    /// The item a column belongs to, and its name there: qualified by an item's name, or a bare
    /// name exactly one item certainly has.
    fn column(&self, c: &protobuf::ColumnRef, catalog: &Catalog) -> Option<(usize, String)> {
        let parts: Vec<&str> = c.fields.iter().filter_map(field_name).collect();
        if parts.len() != c.fields.len() {
            return None;
        }
        match parts.as_slice() {
            [q, column] => self.items.iter().position(|it| it.alias == *q && !it.renamed).map(|i| (i, column.to_string())),
            [column] if self.merged(column) => None,
            [column] => {
                let mut having = (0..self.items.len()).filter(|i| self.has_column(*i, column, catalog));
                match (having.next(), having.next()) {
                    (Some(i), None) => Some((i, column.to_string())),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Whether item `i` certainly has a column of this name.
    fn has_column(&self, i: usize, column: &str, catalog: &Catalog) -> bool {
        let item = &self.items[i];
        let Source::Relation(rv) = &item.source else { return false };
        if item.renamed {
            return false;
        }
        let relation = catalog_name(rv);
        match catalog.column_types.get(&relation) {
            Some(columns) => columns.contains_key(column),
            None => catalog.unique_keys.get(&relation).is_some_and(|keys| keys.iter().flatten().any(|k| k == column)),
        }
    }

    /// Whether every column of item `i` is known, so a name it lacks is certainly not its.
    fn columns_known(&self, i: usize, catalog: &Catalog) -> bool {
        let item = &self.items[i];
        matches!(&item.source, Source::Relation(rv) if !item.renamed && catalog.column_types.contains_key(&catalog_name(rv)))
    }

    /// The type of a column or a scalar subquery on the other side of an equality.
    fn node_type(&self, n: &protobuf::Node, catalog: &Catalog) -> Option<u32> {
        match n.node.as_ref() {
            Some(NodeEnum::ColumnRef(c)) => self.type_of(c, catalog),
            Some(NodeEnum::SubLink(l)) => scalar_type(l, &self.ctes, catalog),
            _ => None,
        }
    }

    /// The type of a column a value reads: of an item here, or of an enclosing query's relation.
    fn type_of(&self, c: &protobuf::ColumnRef, catalog: &Catalog) -> Option<u32> {
        if let Some((i, column)) = self.column(c, catalog) {
            return match &self.items[i].source {
                Source::Relation(rv) => catalog.column_types.get(&catalog_name(rv))?.get(&column).copied(),
                Source::With(cte, _) if cte.aliascolnames.is_empty() => match cte.ctequery.as_deref()?.node.as_ref()? {
                    NodeEnum::SelectStmt(sub) => output_type(sub, &column, &self.ctes, catalog),
                    _ => None,
                },
                Source::Subquery(sub, _) => output_type(sub, &column, &self.ctes, catalog),
                _ => None,
            };
        }
        let local = matches!(c.fields.first().and_then(field_name), Some(q) if self.items.iter().any(|it| it.alias == q));
        if local {
            None
        } else {
            outer_column_type(c, catalog)
        }
    }

    /// The items an expression reads, by index, or `None` when one of its columns cannot be
    /// placed: a bare name more than one relation in scope may hold, or one beside a relation
    /// whose columns the catalog did not give. A column of a subquery's own relations, and one
    /// of an enclosing query (qualified by a name no item here goes by), reads no item.
    fn reads(&self, node: &protobuf::Node, catalog: &Catalog) -> Option<Vec<(usize, Option<String>)>> {
        let tree = serde_json::to_value(node).ok()?;
        let mut out = Vec::new();
        self.walk_reads(&tree, &mut Vec::new(), catalog, &mut out)?;
        Some(out)
    }

    /// `inner` holds the scopes of the subqueries the walk is inside, innermost last: each the
    /// names its FROM goes by, with the relation behind each where it is one the catalog knows.
    fn walk_reads(&self, v: &Value, inner: &mut Vec<Vec<(String, Option<String>)>>, catalog: &Catalog, out: &mut Vec<(usize, Option<String>)>) -> Option<()> {
        match v {
            Value::Object(map) => {
                if let Some(c) = map.get("ColumnRef") {
                    return self.place(c, inner, catalog, out);
                }
                if let Some(select) = map.get("SelectStmt") {
                    inner.push(self.scope_of(select));
                    let walked = select.as_object().into_iter().flat_map(|o| o.values()).try_for_each(|child| self.walk_reads(child, inner, catalog, out));
                    inner.pop();
                    return walked;
                }
                map.values().try_for_each(|child| self.walk_reads(child, inner, catalog, out))
            }
            Value::Array(items) => items.iter().try_for_each(|child| self.walk_reads(child, inner, catalog, out)),
            _ => Some(()),
        }
    }

    /// The names a subquery's FROM goes by, and the catalog relation behind each that is one.
    fn scope_of(&self, select: &Value) -> Vec<(String, Option<String>)> {
        let mut own: Vec<String> = Vec::new();
        for cte in select.get("with_clause").and_then(|w| w.get("ctes")).and_then(Value::as_array).into_iter().flatten() {
            if let Some(name) = cte.get("node").and_then(|n| n.get("CommonTableExpr")).and_then(|c| c.get("ctename")).and_then(Value::as_str) {
                own.push(name.to_string());
            }
        }
        let mut out = Vec::new();
        fn item(v: &Value, ctes: &[String], own: &[String], out: &mut Vec<(String, Option<String>)>) {
            let Some(node) = v.get("node") else { return };
            let alias = |n: &Value| n.get("alias").and_then(|a| a.get("aliasname")).and_then(Value::as_str).map(String::from);
            if let Some(rv) = node.get("RangeVar") {
                let relname = rv.get("relname").and_then(Value::as_str).unwrap_or_default().to_string();
                let schema = rv.get("schemaname").and_then(Value::as_str).unwrap_or_default();
                let renamed = rv.get("alias").and_then(|a| a.get("colnames")).and_then(Value::as_array).is_some_and(|c| !c.is_empty());
                let relation = if schema.is_empty() {
                    (!ctes.contains(&relname) && !own.contains(&relname)).then(|| relname.clone())
                } else {
                    Some(format!("{schema}.{relname}"))
                };
                out.push((alias(rv).unwrap_or(relname), relation.filter(|_| !renamed)));
            } else if let Some(j) = node.get("JoinExpr") {
                for side in ["larg", "rarg"] {
                    if let Some(s) = j.get(side) {
                        item(s, ctes, own, out);
                    }
                }
            } else if let Some(other) = node.as_object().and_then(|o| o.values().next()) {
                out.push((alias(other).unwrap_or_default(), None));
            }
        }
        for f in select.get("from_clause").and_then(Value::as_array).into_iter().flatten() {
            item(f, &self.ctes, &own, &mut out);
        }
        out
    }

    /// Place one column a walk meets, adding the item it reads and the column's name there (`None`
    /// for a whole row, `q.*`) to `out`.
    fn place(&self, c: &Value, inner: &[Vec<(String, Option<String>)>], catalog: &Catalog, out: &mut Vec<(usize, Option<String>)>) -> Option<()> {
        let fields = c.get("fields")?.as_array()?;
        let names: Vec<&str> = fields.iter().filter_map(|f| f.get("node")?.get("String")?.get("sval")?.as_str()).collect();
        match (names.as_slice(), fields.len()) {
            // `q.column`, or `q.*`.
            ([q, rest @ ..], 2) => {
                if inner.iter().rev().any(|scope| scope.iter().any(|(alias, _)| alias == q)) {
                    return Some(());
                }
                if let Some(i) = self.items.iter().position(|it| it.alias == *q) {
                    out.push((i, rest.first().map(|c| c.to_string())));
                }
                Some(())
            }
            ([column], 1) => {
                // A system column (`ctid`) is in no column list, and another value in each row.
                if SYSTEM_COLUMNS.contains(column) || self.merged(column) {
                    return None;
                }
                for scope in inner.iter().rev() {
                    for (_, relation) in scope {
                        let columns = catalog.column_types.get(relation.as_deref()?)?;
                        if columns.contains_key(*column) {
                            return Some(());
                        }
                    }
                }
                let having: Vec<usize> = (0..self.items.len()).filter(|i| self.has_column(*i, column, catalog)).collect();
                match having.as_slice() {
                    [i] => {
                        out.push((*i, Some(column.to_string())));
                        Some(())
                    }
                    // A name no relation in scope has as a column is the whole row of an item
                    // that goes by it, as `(p).author_id` reads `p`: of a subquery the walk is
                    // inside, or of an item here.
                    [] if inner.iter().any(|scope| scope.iter().any(|(alias, _)| alias == column)) => Some(()),
                    [] if self.items.iter().any(|it| it.alias == *column) => {
                        out.extend(self.items.iter().enumerate().filter(|(_, it)| it.alias == *column).map(|(i, _)| (i, None)));
                        Some(())
                    }
                    // Else a column of an enclosing query.
                    [] if (0..self.items.len()).all(|i| self.columns_known(i, catalog)) => Some(()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Why the closure leaves the FROM more than one row, from its first undetermined item.
    fn why_not(&self, determined: &[bool], catalog: &Catalog) -> String {
        let Some(i) = determined.iter().position(|d| !d) else { return "every relation is pinned to one row".into() };
        let item = &self.items[i];
        match &item.source {
            Source::Relation(rv) => {
                let relation = catalog_name(rv);
                let mut shown: Vec<String> = catalog.unique_keys.get(&relation).into_iter().flatten().map(|k| format!("({})", k.join(", "))).collect();
                shown.extend(catalog.partial_keys.get(&relation).into_iter().flatten().map(|(k, p)| format!("({}) where {p}", k.join(", "))));
                match shown.is_empty() {
                    true => format!("`{relation}` has no unique index the engine can read, so nothing bounds the rows a WHERE matches"),
                    false => {
                        if self.items.len() == 1 {
                            format!("the WHERE does not equate every column of a unique key of `{relation}` — it has {}", shown.join(", "))
                        } else {
                            format!(
                                "a join returns a row per pair it matches, and the WHERE and ON clauses do not equate every column of a unique key of `{}` to values or to columns of relations pinned to one row — it has {}",
                                item.alias,
                                shown.join(", ")
                            )
                        }
                    }
                }
            }
            Source::With(cte, _) => format!("the WITH query `{}` returns any number of rows", cte.ctename),
            Source::Enclosing => format!("`{}` is a WITH query of an enclosing statement, which returns any number of rows", item.alias),
            Source::Subquery(..) => format!("the subquery `{}` returns any number of rows", item.alias),
            Source::Opaque => format!("`{}` in the FROM is a function or table function, which returns any number of rows", item.alias),
        }
    }
}

/// The name of the first function call in a parse tree, as JSON.
fn first_function_name(v: &Value) -> Option<String> {
    match v {
        Value::Object(map) => map.get("FuncCall").and_then(|f| last_name(f.get("funcname"))).or_else(|| map.values().find_map(first_function_name)),
        Value::Array(items) => items.iter().find_map(first_function_name),
        _ => None,
    }
}

/// Whether a WITH query returns one row at most: a SELECT the proof bounds, or a write it does.
fn with_query_at_most_one(cte: &protobuf::CommonTableExpr, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> bool {
    match cte.ctequery.as_deref().and_then(|q| q.node.as_ref()) {
        Some(NodeEnum::SelectStmt(inner)) => at_most_one_select(inner, dialect, trust, catalog).is_ok(),
        Some(body @ (NodeEnum::InsertStmt(_) | NodeEnum::UpdateStmt(_) | NodeEnum::DeleteStmt(_))) => matches!(dml_node(body, catalog), Some(Ok(_))),
        _ => false,
    }
}

/// Whether `node` may give each row another value: a call of a function with a volatile overload
/// (`random()`, `nextval`) or of one the catalog does not know, whose volatility it cannot tell;
/// an operator other than a built-in spelling no user defined, which a volatile function may
/// implement (`id = (0 +? 0)`); or a cast that may call a user's cast function.
fn calls_volatile(node: &protobuf::Node, catalog: &Catalog) -> bool {
    let Ok(tree) = serde_json::to_value(node) else { return true };
    fn walk(v: &Value, catalog: &Catalog) -> bool {
        match v {
            Value::Object(map) => map.iter().any(|(key, child)| {
                let volatile = match key.as_str() {
                    "FuncCall" => last_name(child.get("funcname")).is_none_or(|n| !catalog.functions.contains(&n) || catalog.volatile.contains(&n)),
                    "AExpr" => operator_names(child).iter().any(|n| !trusted_operator(n, catalog)),
                    "TypeCast" => user_cast(child, catalog),
                    _ => false,
                };
                volatile || walk(child, catalog)
            }),
            Value::Array(items) => items.iter().any(|i| walk(i, catalog)),
            _ => false,
        }
    }
    walk(&tree, catalog)
}

/// The operators an `A_Expr` applies, by name: its own, or `<=` and `>=` for a BETWEEN.
fn operator_names(aexpr: &Value) -> Vec<String> {
    use protobuf::AExprKind;
    let between = [AExprKind::AexprBetween, AExprKind::AexprNotBetween, AExprKind::AexprBetweenSym, AExprKind::AexprNotBetweenSym].map(|k| k as i64);
    if aexpr.get("kind").and_then(Value::as_i64).is_some_and(|k| between.contains(&k)) {
        return vec!["<=".into(), ">=".into()];
    }
    last_name(aexpr.get("name")).into_iter().collect()
}

/// Whether an operator is one of the built-in spellings the trust check asks about, and no user
/// or extension defined one of that name: the built-in operators are never volatile.
fn trusted_operator(name: &str, catalog: &Catalog) -> bool {
    (infer::POSTGRES_OPERATORS.contains(&name) || infer::POSTGRES_STRICT_OPERATORS.contains(&name)) && !catalog.user_operators.contains(name)
}

/// Whether a cast may call a user's cast function: one to a type outside the built-in targets no
/// such function has, of anything but a string literal or a parameter, which the type's input
/// function reads and the server types as the target.
fn user_cast(cast: &Value, catalog: &Catalog) -> bool {
    let Some(targets) = &catalog.cast_targets else { return false };
    if last_name(cast.get("type_name").and_then(|t| t.get("names"))).is_some_and(|n| targets.contains(&n)) {
        return false;
    }
    let arg = cast.get("arg").and_then(|a| a.get("node"));
    !arg.is_some_and(|a| a.get("ParamRef").is_some() || a.get("AConst").and_then(|c| c.get("val")).is_some_and(|v| v.get("Sval").is_some()))
}

/// The type of a column of an enclosing query, through [`Catalog::outer_aliases`].
fn outer_column_type(c: &protobuf::ColumnRef, catalog: &Catalog) -> Option<u32> {
    let parts: Vec<&str> = c.fields.iter().filter_map(field_name).collect();
    let [q, column] = parts.as_slice() else { return None };
    let relation = catalog.outer_aliases.get(*q)?;
    catalog.column_types.get(relation)?.get(*column).copied()
}

/// A node as JSON with what the comparison of two spellings must ignore: every `location`, the
/// qualifier `alias` on a column (`u.email` is `email` when read for `u`), and a cast to a text
/// type around a string literal, which the server prints for a literal the statement may leave
/// to take the column's type.
fn normalized(node: &protobuf::Node, alias: &str) -> Value {
    fn walk(v: &mut Value, alias: &str) {
        match v {
            Value::Object(map) => {
                map.remove("location");
                if let Some(fields) = map.get_mut("ColumnRef").and_then(|c| c.get_mut("fields")).and_then(Value::as_array_mut) {
                    let qualified = fields.len() == 2 && fields[0].get("node").and_then(|n| n.get("String")).and_then(|s| s.get("sval")).and_then(Value::as_str) == Some(alias);
                    if qualified {
                        fields.remove(0);
                    }
                }
                let literal = map.get("TypeCast").and_then(|t| {
                    let arg = t.get("arg")?.get("node")?;
                    let name = t.get("type_name")?.get("names")?.as_array()?.last()?.get("node")?.get("String")?.get("sval")?.as_str()?;
                    let text = ["text", "varchar", "bpchar"].contains(&name) && arg.get("AConst").and_then(|c| c.get("val")).is_some_and(|v| v.get("Sval").is_some());
                    text.then(|| arg.clone())
                });
                if let Some(arg) = literal {
                    map.remove("TypeCast");
                    if let Value::Object(inner) = arg {
                        map.extend(inner);
                    }
                }
                map.values_mut().for_each(|child| walk(child, alias));
            }
            Value::Array(items) => items.iter_mut().for_each(|child| walk(child, alias)),
            _ => {}
        }
    }
    let mut tree = serde_json::to_value(node).unwrap_or(Value::Null);
    walk(&mut tree, alias);
    tree
}

/// Whether `side` spells the expression of an index key part (`(lower(email))`) for the relation
/// read as `alias`: the same parse tree once [`normalized`].
fn matches_expression(side: &protobuf::Node, part: &str, alias: &str) -> bool {
    let Ok(parsed) = pg_query::parse(&format!("SELECT {part}")) else { return false };
    let Some(NodeEnum::SelectStmt(select)) = parsed.protobuf.stmts.first().and_then(|s| s.stmt.as_deref()).and_then(|n| n.node.as_ref()) else { return false };
    let Some(NodeEnum::ResTarget(target)) = select.target_list.first().and_then(|t| t.node.as_ref()) else { return false };
    let Some(expression) = target.val.as_deref() else { return false };
    normalized(side, alias) == normalized(expression, alias)
}

/// The column of a normalized `col IS NOT NULL`.
fn not_null_column(v: &Value) -> Option<Value> {
    let test = v.get("node")?.get("NullTest")?;
    let not_null = test.get("nulltesttype").and_then(Value::as_i64) == Some(protobuf::NullTestType::IsNotNull as i64);
    let arg = test.get("arg")?;
    (not_null && arg.get("node")?.get("ColumnRef").is_some()).then(|| arg.clone())
}

/// Whether a normalized conjunct is `column = …` or `… = column`.
fn equates_column(v: &Value, column: &Value) -> bool {
    let Some(e) = v.get("node").and_then(|n| n.get("AExpr")) else { return false };
    let equality = e.get("name").and_then(Value::as_array).and_then(|n| n.last()).and_then(|n| n.get("node")?.get("String")?.get("sval")?.as_str()) == Some("=")
        && e.get("kind").and_then(Value::as_i64) == Some(protobuf::AExprKind::AexprOp as i64);
    equality && (e.get("lexpr") == Some(column) || e.get("rexpr") == Some(column))
}

/// The one relation a SELECT reads, by the name the catalog keys it under, with the name its
/// columns go by: a plain SELECT over one table or view, not a WITH query (`ctes`).
fn only_relation<'s>(sub: &'s SelectStmt, ctes: &[String]) -> Option<(String, &'s str)> {
    if sub.with_clause.is_some() || sub.larg.is_some() || !sub.values_lists.is_empty() {
        return None;
    }
    let [only] = sub.from_clause.as_slice() else { return None };
    let Some(NodeEnum::RangeVar(rv)) = only.node.as_ref() else { return None };
    if rv.alias.as_ref().is_some_and(|a| !a.colnames.is_empty()) || (rv.schemaname.is_empty() && ctes.contains(&rv.relname)) {
        return None;
    }
    Some((catalog_name(rv), rv.alias.as_ref().map_or(rv.relname.as_str(), |a| a.aliasname.as_str())))
}

/// The type of a column of the one relation `sub` reads, named by `c`: bare, or qualified by the
/// relation's name.
fn projected(sub: &SelectStmt, c: &protobuf::ColumnRef, ctes: &[String], catalog: &Catalog) -> Option<u32> {
    let (relation, alias) = only_relation(sub, ctes)?;
    let parts: Vec<&str> = c.fields.iter().filter_map(field_name).collect();
    let column = match parts.as_slice() {
        [column] => *column,
        [q, column] if *q == alias => *column,
        _ => return None,
    };
    catalog.column_types.get(&relation)?.get(column).copied()
}

/// The type of the output column `name` of a subquery or WITH query over one relation, where the
/// column is one of the relation's, passed through: `*`, or a plain column under its own name or
/// an alias. A grouping or an aggregate changes no plain column's type.
fn output_type(sub: &SelectStmt, name: &str, ctes: &[String], catalog: &Catalog) -> Option<u32> {
    for target in &sub.target_list {
        let Some(NodeEnum::ResTarget(r)) = target.node.as_ref() else { continue };
        let Some(NodeEnum::ColumnRef(c)) = r.val.as_deref().and_then(|v| v.node.as_ref()) else { continue };
        if c.fields.last().is_some_and(|f| matches!(f.node, Some(NodeEnum::AStar(_)))) {
            let (relation, _) = only_relation(sub, ctes)?;
            if let Some(t) = catalog.column_types.get(&relation).and_then(|types| types.get(name)) {
                return Some(*t);
            }
            continue;
        }
        let own = c.fields.iter().filter_map(field_name).last();
        let out = if r.name.is_empty() { own } else { Some(r.name.as_str()) };
        if out == Some(name) {
            return projected(sub, c, ctes, catalog);
        }
    }
    None
}

/// The type of a scalar subquery's one column, where it is a column of the one relation the
/// subquery reads: `(SELECT author_id FROM posts WHERE id = $1)`.
fn scalar_type(l: &protobuf::SubLink, ctes: &[String], catalog: &Catalog) -> Option<u32> {
    if l.sub_link_type != protobuf::SubLinkType::ExprSublink as i32 {
        return None;
    }
    let Some(NodeEnum::SelectStmt(sub)) = l.subselect.as_deref()?.node.as_ref() else { return None };
    let [target] = sub.target_list.as_slice() else { return None };
    let Some(NodeEnum::ResTarget(r)) = target.node.as_ref() else { return None };
    let Some(NodeEnum::ColumnRef(c)) = r.val.as_deref()?.node.as_ref() else { return None };
    projected(sub, c, ctes, catalog)
}

/// Every alias a statement gives a relation, through every subquery, to the relation's name as
/// [`Catalog::unique_keys`] keys it: what a host puts in [`Catalog::outer_aliases`] before it
/// proves a subquery of the statement on its own. An alias given to two relations is left out.
pub fn aliases(sql: &str) -> std::collections::HashMap<String, String> {
    let mut out: std::collections::HashMap<String, Option<String>> = std::collections::HashMap::new();
    let Ok(parsed) = pg_query::parse(sql) else { return Default::default() };
    let Ok(tree) = serde_json::to_value(&parsed.protobuf) else { return Default::default() };
    fn walk(v: &Value, out: &mut std::collections::HashMap<String, Option<String>>) {
        match v {
            Value::Object(map) => {
                if let Some(rv) = map.get("RangeVar") {
                    let relname = rv.get("relname").and_then(Value::as_str).unwrap_or_default().to_string();
                    let schema = rv.get("schemaname").and_then(Value::as_str).unwrap_or_default();
                    let alias = rv.get("alias").and_then(|a| a.get("aliasname")).and_then(Value::as_str).map_or_else(|| relname.clone(), String::from);
                    let relation = if schema.is_empty() { relname } else { format!("{schema}.{relname}") };
                    out.entry(alias).and_modify(|known| {
                        if known.as_deref() != Some(relation.as_str()) {
                            *known = None;
                        }
                    }).or_insert(Some(relation));
                }
                map.values().for_each(|child| walk(child, out));
            }
            Value::Array(items) => items.iter().for_each(|child| walk(child, out)),
            _ => {}
        }
    }
    walk(&tree, &mut out);
    out.into_iter().filter_map(|(alias, relation)| Some((alias, relation?))).collect()
}

/// Which of the key proof's structural conditions a statement breaks, if it breaks one. A shape
/// that returns a row per group or per arm is not a shape a unique key speaks for.
fn not_key_shaped(s: &SelectStmt) -> Option<&'static str> {
    if s.larg.is_some() || s.rarg.is_some() {
        Some("a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows")
    } else if !s.values_lists.is_empty() {
        Some("a VALUES list returns a row per row it lists")
    } else if !s.group_clause.is_empty() {
        Some("GROUP BY returns one row per group")
    } else if s.having_clause.is_some() {
        Some("HAVING leaves a row per group")
    } else if s.into_clause.is_some() {
        Some("SELECT INTO creates a table and returns no rows")
    } else {
        None
    }
}

/// The key proof over the whole FROM: every item is determined ([`From`]), so the FROM has one
/// row at most; or `SELECT DISTINCT` (`DISTINCT ON`) over values that are the same in every row.
fn one_row(s: &SelectStmt, dialect: Dialect, trust: &Trust, catalog: &Catalog) -> Result<(), String> {
    if s.from_clause.is_empty() {
        return Err("a SELECT with no FROM and a WHERE can return no row or one, and the engine proves that only through a LIMIT".into());
    }
    let from = From::read(s, catalog);
    if let Some(why) = from.refused {
        return Err(why.to_string());
    }
    let determined = from.determined(false, dialect, trust, catalog);
    if determined.iter().all(|d| *d) {
        return Ok(());
    }
    // `SELECT DISTINCT` returns one row when every output value is the same in every row, and
    // `DISTINCT ON` when every expression it is distinct on is.
    if !s.distinct_clause.is_empty() {
        let constant = from.determined(true, dialect, trust, catalog);
        let on: Vec<&protobuf::Node> = s.distinct_clause.iter().filter(|d| d.node.is_some()).collect();
        let values: Vec<&protobuf::Node> = if on.is_empty() {
            s.target_list
                .iter()
                .filter_map(|t| match t.node.as_ref() {
                    Some(NodeEnum::ResTarget(r)) => r.val.as_deref(),
                    _ => None,
                })
                .collect()
        } else {
            on
        };
        if !values.is_empty() && values.iter().all(|e| from.pinned(e, &constant, catalog)) {
            return Ok(());
        }
    }
    Err(from.why_not(&determined, catalog))
}

/// Whether `where_clause` pins one row of `rv` by a unique key, as the closure over one relation
/// does ([`From::single`]).
fn key_bounds(
    rv: &protobuf::RangeVar,
    with: Option<&protobuf::WithClause>,
    where_clause: Option<&protobuf::Node>,
    catalog: &Catalog,
) -> Result<(), String> {
    let relation = catalog_name(rv);
    // A CTE of the same name shadows the table, and a CTE has no index at all.
    if let Some(with) = with {
        if rv.schemaname.is_empty() && with.ctes.iter().any(|c| cte_name(c) == Some(&relation)) {
            return Err(format!("`{relation}` here is the WITH query of that name, which has no unique index"));
        }
    }
    let from = From::single(rv, where_clause, with);
    let determined = from.determined(false, Dialect::Postgres, &Trust::default(), catalog);
    if determined.iter().all(|d| *d) {
        Ok(())
    } else {
        Err(from.why_not(&determined, catalog))
    }
}

/// How many rows a data-modifying statement's `RETURNING` produces, from its shape: an `INSERT`
/// of one `VALUES` row is one row (or none under `ON CONFLICT DO NOTHING`); an `UPDATE` or
/// `DELETE` whose `WHERE` pins a unique key of its one target relation is at most one; anything
/// else is any number. `None` when the statement is not one of these.
pub fn dml(sql: &str, catalog: &Catalog) -> Option<Result<Cardinality, String>> {
    let parsed = pg_query::parse(sql).ok()?;
    let mut stmts = parsed.protobuf.stmts;
    if stmts.len() != 1 {
        return None;
    }
    let node = stmts.pop().and_then(|s| s.stmt).and_then(|n| n.node)?;
    dml_node(&node, catalog)
}

fn dml_node(node: &NodeEnum, catalog: &Catalog) -> Option<Result<Cardinality, String>> {
    Some(match node {
        NodeEnum::InsertStmt(i) => {
            let rows = match i.select_stmt.as_deref().and_then(|n| n.node.as_ref()) {
                Some(NodeEnum::SelectStmt(s)) if s.values_lists.len() == 1 && s.from_clause.is_empty() => 1,
                Some(NodeEnum::SelectStmt(s)) if !s.values_lists.is_empty() => s.values_lists.len(),
                // `INSERT … DEFAULT VALUES` has no select at all.
                None => 1,
                _ => return Some(Err("an INSERT … SELECT returns a row per row the SELECT produces".into())),
            };
            if rows != 1 {
                return Some(Err("a VALUES list returns a row per row it lists".into()));
            }
            if rewritten(i.relation.as_ref(), TRIGGER_ROW_INSERT, catalog) {
                return Some(Err(REWRITTEN.into()));
            }
            // A row trigger that runs BEFORE the insert (or the update ON CONFLICT turns it into)
            // or INSTEAD of it can return NULL, and then the row is neither written nor returned.
            let skipped = |events: i32| i.relation.as_ref().and_then(|rv| catalog.skipping_triggers.get(&catalog_name(rv))).is_some_and(|mask| mask & events != 0);
            match i.on_conflict_clause.as_deref() {
                Some(c) if c.action == protobuf::OnConflictAction::OnconflictNothing as i32 => Ok(Cardinality::AtMostOne),
                // `DO UPDATE … WHERE` leaves a conflicting row alone, and returns nothing for it.
                Some(c) if c.action == protobuf::OnConflictAction::OnconflictUpdate as i32 && (c.where_clause.is_some() || skipped(TRIGGER_ROW_INSERT | TRIGGER_ROW_UPDATE)) => Ok(Cardinality::AtMostOne),
                _ if skipped(TRIGGER_ROW_INSERT) => Ok(Cardinality::AtMostOne),
                _ => Ok(Cardinality::One),
            }
        }
        NodeEnum::UpdateStmt(u) => {
            if !u.from_clause.is_empty() {
                return Some(Err("an UPDATE … FROM returns a row per pair it matches".into()));
            }
            let rv = u.relation.as_ref()?;
            if rewritten(Some(rv), TRIGGER_ROW_UPDATE, catalog) {
                return Some(Err(REWRITTEN.into()));
            }
            key_bounds(rv, u.with_clause.as_ref(), u.where_clause.as_deref(), catalog).map(|()| Cardinality::AtMostOne)
        }
        NodeEnum::DeleteStmt(d) => {
            if !d.using_clause.is_empty() {
                return Some(Err("a DELETE … USING returns a row per pair it matches".into()));
            }
            let rv = d.relation.as_ref()?;
            if rewritten(Some(rv), TRIGGER_ROW_DELETE, catalog) {
                return Some(Err(REWRITTEN.into()));
            }
            key_bounds(rv, d.with_clause.as_ref(), d.where_clause.as_deref(), catalog).map(|()| Cardinality::AtMostOne)
        }
        _ => return None,
    })
}

/// What the proofs settle on, for one statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    One,
    AtMostOne,
}

/// Why a write to a relation with a rewrite rule for it, or to a view, is any number of rows.
const REWRITTEN: &str = "a rewrite rule replaces the write with actions of its own, and a view hands it to a table whose triggers and rules run, so RETURNING can return any number of rows";

/// Whether a rewrite rule (or the relation being a view) covers one of these events.
fn rewritten(rv: Option<&protobuf::RangeVar>, events: i32, catalog: &Catalog) -> bool {
    rv.and_then(|rv| catalog.rewritten.get(&catalog_name(rv))).is_some_and(|mask| mask & events != 0)
}

fn cte_name(cte: &protobuf::Node) -> Option<&String> {
    match cte.node.as_ref() {
        Some(NodeEnum::CommonTableExpr(c)) => Some(&c.ctename),
        _ => None,
    }
}

/// The name a relation goes by in the catalog the proof reads: schema-qualified when the
/// statement qualifies it, so `archive.users` never takes the keys of the `users` on the path.
fn catalog_name(rv: &protobuf::RangeVar) -> String {
    if rv.schemaname.is_empty() {
        rv.relname.clone()
    } else {
        format!("{}.{}", rv.schemaname, rv.relname)
    }
}

/// Whether `column = value` restricts `relation` the way its unique index does, so the key it
/// belongs to may count as pinned. The planner asks that the comparison be the index's own
/// equality, under a collation that agrees with the index's (`relation_has_unique_index_ext`):
///
/// - An explicit `COLLATE` anywhere in the value may make the comparison case- or
///   accent-insensitive, matching rows the index tells apart.
/// - A value of another type than the column's makes the server convert one side. Converting the
///   value is harmless, but a conversion of the column to the value's type can map two of its
///   values to one (`numeric_col = $1::float8`: 0.1 and 0.10000000000000000001 are one `float8`).
///   So the value's type must be one the column's operator family compares as it is
///   ([`compatible`]). A literal or a parameter the server types from the column adapts to it.
/// - Where a user or an extension defined an operator named `=`, a comparison across two types may
///   resolve to theirs, so only a value of the column's own type, or one that adapts to it, counts.
/// - A value whose type the build cannot name counts only for a column whose type no implicit cast
///   maps to a wider one with loss ([`ANY_VALUE_TYPES`]). A column, or a scalar subquery's
///   column, on the other side has the type `type_of` gives it, from the catalog.
/// - A volatile call is evaluated for each row, so `id = floor(random() * 3)::int` can match
///   several rows ([`calls_volatile`]).
fn equality_bounds(catalog: &Catalog, relation: &str, column: &str, value: &protobuf::Node, type_of: &dyn Fn(&protobuf::Node) -> Option<u32>) -> bool {
    if mentions(value, "CollateClause") || calls_volatile(value, catalog) {
        return false;
    }
    let Some(column_type) = catalog.column_types.get(relation).and_then(|types| types.get(column)).copied() else { return true };
    match value_type(value, catalog, type_of) {
        ValueType::Adapts => true,
        ValueType::Is(t) => t == column_type || (!catalog.equality_shadowed && compatible(column_type, t)),
        ValueType::Unnamed => !catalog.equality_shadowed && ANY_VALUE_TYPES.contains(&column_type),
    }
}

/// What the build knows of a compared value's type.
enum ValueType {
    /// The server types it from the column it is compared with: a string literal, an uncast
    /// parameter whose describe the build does not have.
    Adapts,
    Is(u32),
    Unnamed,
}

fn value_type(value: &protobuf::Node, catalog: &Catalog, type_of: &dyn Fn(&protobuf::Node) -> Option<u32>) -> ValueType {
    match value.node.as_ref() {
        Some(NodeEnum::ColumnRef(_)) | Some(NodeEnum::SubLink(_)) => type_of(value).map_or(ValueType::Unnamed, ValueType::Is),
        Some(NodeEnum::AConst(c)) => match &c.val {
            Some(protobuf::a_const::Val::Ival(_)) => ValueType::Is(oid::INT4),
            Some(protobuf::a_const::Val::Fval(f)) => ValueType::Is(if f.fval.parse::<i64>().is_ok() { oid::INT8 } else { oid::NUMERIC }),
            Some(protobuf::a_const::Val::Boolval(_)) => ValueType::Is(oid::BOOL),
            _ => ValueType::Adapts,
        },
        Some(NodeEnum::ParamRef(p)) => match catalog.param_types.get((p.number as usize).wrapping_sub(1)) {
            Some(&t) if t != 0 => ValueType::Is(t),
            _ => ValueType::Adapts,
        },
        Some(NodeEnum::TypeCast(t)) => {
            let name = t.type_name.as_ref().filter(|n| n.array_bounds.is_empty() && !n.setof && !n.pct_type).and_then(|n| last_string(&n.names));
            match name.and_then(|n| CAST_TYPES.iter().find(|(c, _)| *c == n).map(|(_, o)| *o)) {
                Some(o) => ValueType::Is(o),
                None => ValueType::Unnamed,
            }
        }
        _ => ValueType::Unnamed,
    }
}

/// Built-in cast targets by the name libpg_query gives them.
const CAST_TYPES: &[(&str, u32)] = &[
    ("text", oid::TEXT),
    ("varchar", oid::VARCHAR),
    ("bpchar", oid::BPCHAR),
    ("name", oid::NAME),
    ("int2", oid::INT2),
    ("int4", oid::INT4),
    ("int8", oid::INT8),
    ("numeric", oid::NUMERIC),
    ("float4", oid::FLOAT4),
    ("float8", oid::FLOAT8),
    ("bool", oid::BOOL),
    ("uuid", oid::UUID),
    ("date", oid::DATE),
    ("timestamp", oid::TIMESTAMP),
    ("timestamptz", oid::TIMESTAMPTZ),
];

/// Column types that no implicit cast converts with loss, so a value of any type bounds them:
/// the server converts the value, or converts the column exactly (`int4` to `float8`, `date` to
/// `timestamp`, `varchar` to `text`), or refuses the comparison.
const ANY_VALUE_TYPES: &[u32] = &[oid::INT2, oid::INT4, oid::BOOL, oid::UUID, oid::DATE, oid::TEXT, oid::VARCHAR];

/// Whether a column of type `column` compared with a value of type `value` uses an equality of the
/// column's btree operator family on the column as it is: the cross-type members of `integer_ops`,
/// `float_ops`, `text_ops` and `datetime_ops`, or a value the server converts to the column's type
/// (an integer to `numeric`, an integer or a `numeric` to a float). One member of `datetime_ops`
/// converts the column: `timestamp = timestamptz` reads the timestamp in the session's time zone,
/// where two wall-clock times are one instant (02:30 and 03:30 on a day that skips 02:00).
fn compatible(column: u32, value: u32) -> bool {
    const INTEGERS: &[u32] = &[oid::INT2, oid::INT4, oid::INT8];
    const FLOATS: &[u32] = &[oid::FLOAT4, oid::FLOAT8];
    const STRINGS: &[u32] = &[oid::TEXT, oid::VARCHAR, oid::NAME];
    const DATES: &[u32] = &[oid::DATE, oid::TIMESTAMP, oid::TIMESTAMPTZ];
    let both = |set: &[u32]| set.contains(&column) && set.contains(&value);
    column == value
        || both(INTEGERS)
        || both(FLOATS)
        || both(STRINGS)
        || (both(DATES) && !(column == oid::TIMESTAMP && value == oid::TIMESTAMPTZ))
        || (column == oid::NUMERIC && INTEGERS.contains(&value))
        || (FLOATS.contains(&column) && (INTEGERS.contains(&value) || value == oid::NUMERIC))
}

/// Column types whose equality is identity: two equal values are the same value, so anything
/// computed from one is computed from the other. Not `numeric` (1 = 1.0, of another scale), not
/// the floats (0 = -0), and not a string under a nondeterministic collation ('a' = 'A').
const IDENTITY_TYPES: &[u32] = &[oid::INT2, oid::INT4, oid::INT8, oid::BOOL, oid::UUID, oid::DATE, oid::TIMESTAMP, oid::TIMESTAMPTZ, oid::TEXT, oid::VARCHAR, oid::NAME];

/// Whether this column's equality is identity ([`IDENTITY_TYPES`]), as far as the catalog says.
fn identity(catalog: &Catalog, relation: &str, column: &str) -> bool {
    catalog.column_types.get(relation).and_then(|types| types.get(column)).is_some_and(|t| IDENTITY_TYPES.contains(t))
        && !catalog.nondeterministic.get(relation).is_some_and(|columns| columns.contains(column))
}

/// The system columns every table has and no column list names.
const SYSTEM_COLUMNS: &[&str] = &["ctid", "xmin", "xmax", "cmin", "cmax", "tableoid"];

/// Whether a window function (a call with `OVER`) sits anywhere in `node`.
fn mentions_window(node: &protobuf::Node) -> bool {
    let Ok(tree) = serde_json::to_value(node) else { return true };
    fn walk(v: &Value) -> bool {
        match v {
            Value::Object(map) => map.iter().any(|(key, child)| (key == "FuncCall" && child.get("over").is_some_and(|o| !o.is_null())) || walk(child)),
            Value::Array(items) => items.iter().any(walk),
            _ => false,
        }
    }
    walk(&tree)
}

/// Whether a node of this kind sits anywhere in `node`.
fn mentions(node: &protobuf::Node, kind: &str) -> bool {
    let Ok(tree) = serde_json::to_value(node) else { return true };
    fn walk(v: &Value, kind: &str) -> bool {
        match v {
            Value::Object(map) => map.iter().any(|(key, child)| key == kind || walk(child, kind)),
            Value::Array(items) => items.iter().any(|i| walk(i, kind)),
            _ => false,
        }
    }
    walk(&tree, kind)
}

/// Whether this is the equality operator itself, rather than `= ANY (…)`, `IS NOT DISTINCT FROM`,
/// or an operator merely spelled with an equals sign.
fn is_equality(e: &protobuf::AExpr) -> bool {
    e.kind == protobuf::AExprKind::AexprOp as i32 && e.name.last().and_then(field_name) == Some("=")
}

/// The column name this expression is, when it names a column of `alias` — `alias.col`, or a bare
/// `col`, which SQL resolves to the innermost relation that has it, and a unique key's column is
/// one this relation has.
fn own_column(node: &protobuf::Node, alias: &str) -> Option<String> {
    let Some(NodeEnum::ColumnRef(c)) = node.node.as_ref() else { return None };
    let parts: Vec<&str> = c.fields.iter().filter_map(field_name).collect();
    match parts.as_slice() {
        [column] => Some((*column).to_string()),
        [qualifier, column] if *qualifier == alias => Some((*column).to_string()),
        _ => None,
    }
}

fn field_name(node: &protobuf::Node) -> Option<&str> {
    match node.node.as_ref() {
        Some(NodeEnum::String(s)) => Some(s.sval.as_str()),
        _ => None,
    }
}

/// A constant `LIMIT n`, when the statement has one the build can read.
fn const_limit(s: &SelectStmt) -> Option<i64> {
    // `FETCH FIRST n ROWS WITH TIES` also returns every row that ties with the last one, so its
    // count bounds nothing.
    if s.limit_option == protobuf::LimitOption::WithTies as i32 {
        return None;
    }
    match s.limit_count.as_deref()?.node.as_ref()? {
        NodeEnum::AConst(c) if !c.isnull => match &c.val {
            Some(protobuf::a_const::Val::Ival(i)) => Some(i.ival as i64),
            _ => None,
        },
        _ => None,
    }
}

/// Whether one rewritten statement provably returns no row, with the reason: a constant `LIMIT 0`
/// (`FETCH FIRST 0 ROWS ONLY` is the same node). Nothing else is claimed, so an endpoint whose
/// `returns:` expects a row is refused only over a statement that can never have one.
pub fn never_a_row(sql: &str, dialect: Dialect) -> Option<&'static str> {
    let s = one_select(sql, dialect).ok()?;
    match s.limit_count.as_deref()?.node.as_ref()? {
        NodeEnum::AConst(c) if !c.isnull => match &c.val {
            Some(protobuf::a_const::Val::Ival(i)) if i.ival == 0 => Some("LIMIT 0 returns no rows"),
            _ => None,
        },
        _ => None,
    }
}

/// The one `SELECT` a statement is, parsed as the dialect's statements are read.
fn one_select(sql: &str, dialect: Dialect) -> Result<Box<SelectStmt>, String> {
    let text = match dialect {
        Dialect::Postgres => sql.to_string(),
        Dialect::Sqlite | Dialect::Mysql => infer::dollar_placeholders(sql),
    };
    let parsed = pg_query::parse(&text).map_err(|_| match dialect {
        Dialect::Postgres => "the statement does not parse".to_string(),
        _ => format!("the engine reads a {} statement's shape in the SQL it shares with Postgres, and this one uses syntax outside it", dialect.name()),
    })?;
    let mut stmts = parsed.protobuf.stmts;
    if stmts.len() != 1 {
        return Err("it is not exactly one statement".into());
    }
    match stmts.pop().and_then(|s| s.stmt).and_then(|n| n.node) {
        Some(NodeEnum::SelectStmt(s)) => Ok(s),
        _ => Err("only a SELECT is proven".into()),
    }
}

/// Which of `single_row`'s conditions a statement breaks, for the diagnostic. The decision is
/// `single_row`'s; this only names it.
fn not_one_row_shaped(s: &SelectStmt) -> &'static str {
    if s.larg.is_some() || s.rarg.is_some() {
        "a set operation (UNION, INTERSECT, EXCEPT) returns any number of rows"
    } else if !s.values_lists.is_empty() {
        "a VALUES list returns a row per row it lists"
    } else if !s.group_clause.is_empty() {
        "GROUP BY returns one row per group, and none when nothing matches"
    } else if s.having_clause.is_some() {
        "HAVING can remove the row"
    } else {
        "a LIMIT or an OFFSET can remove the row (LIMIT 0, OFFSET 1)"
    }
}

/// The function and operator names of one query level.
#[derive(Debug, Default)]
struct Calls {
    functions: Vec<String>,
    operators: Vec<String>,
}

impl Calls {
    /// Why a call here may return a set, if one may.
    fn set_returning(&self, catalog: &Catalog) -> Option<String> {
        for name in &self.functions {
            if catalog.set_returning.contains(name) {
                return Some(format!("the set-returning function {name} returns a row per value, and none for an empty set"));
            }
            if !catalog.functions.contains(name) {
                return Some(format!("the function {name} is not in the catalog, so the build cannot rule out that it returns a set"));
            }
        }
        self.operators
            .iter()
            .find(|op| catalog.set_returning_operators.contains(op.as_str()))
            .map(|op| format!("the operator {op} is implemented by a set-returning function in this database"))
    }
}

fn calls_at_this_level<'a>(nodes: impl Iterator<Item = &'a protobuf::Node>) -> Calls {
    let mut calls = Calls::default();
    for node in nodes {
        if let Ok(tree) = serde_json::to_value(node) {
            walk(&tree, &mut calls);
        }
    }
    calls
}

fn last_name(names: Option<&Value>) -> Option<String> {
    names?.as_array()?.last()?.get("node")?.get("String")?.get("sval")?.as_str().map(String::from)
}

fn walk(v: &Value, calls: &mut Calls) {
    match v {
        Value::Object(map) => {
            for (key, child) in map {
                match key.as_str() {
                    // A subquery answers one value to this level; only the expression it tests
                    // (`x IN (SELECT …)`) and the operator it tests with are evaluated here.
                    "SubLink" => {
                        let tests = child.get("testexpr").is_some_and(|t| !t.is_null());
                        match last_name(child.get("oper_name")) {
                            Some(op) => calls.operators.push(op),
                            None if tests => calls.operators.push("=".into()),
                            None => {}
                        }
                        if let Some(test) = child.get("testexpr") {
                            walk(test, calls);
                        }
                    }
                    "FuncCall" => {
                        if let Some(name) = last_name(child.get("funcname")) {
                            calls.functions.push(name);
                        }
                        walk(child, calls);
                    }
                    "AExpr" => {
                        match last_name(child.get("name")) {
                            // `a BETWEEN x AND y` is spelled with the comparisons it expands to.
                            Some(op) if op.contains("BETWEEN") => calls.operators.extend(["<", ">", "<=", ">="].map(String::from)),
                            Some(op) => calls.operators.push(op),
                            None => {}
                        }
                        walk(child, calls);
                    }
                    _ => walk(child, calls),
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|i| walk(i, calls)),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> Catalog {
        let mut c = Catalog::default();
        c.unique_keys.insert("users".into(), vec![vec!["id".into()], vec!["email".into()]]);
        c.unique_keys.insert("posts".into(), vec![vec!["id".into()]]);
        for f in ["count", "lower", "abs", "floor"] {
            c.functions.insert(f.into());
        }
        c
    }

    /// `catalog()` with every column's type, as a Postgres host reports them, and more relations.
    fn joined() -> Catalog {
        let mut c = catalog();
        c.unique_keys.insert("comments".into(), vec![vec!["id".into()]]);
        c.unique_keys.insert("profiles".into(), vec![vec!["user_id".into()]]);
        let columns = |cols: &[(&str, u32)]| cols.iter().map(|(n, t)| (n.to_string(), *t)).collect();
        c.column_types.insert("users".into(), columns(&[("id", oid::INT8), ("email", oid::TEXT), ("name", oid::TEXT), ("pinned_post", oid::INT8)]));
        c.column_types.insert("posts".into(), columns(&[("id", oid::INT8), ("author_id", oid::INT8), ("title", oid::TEXT), ("score", oid::FLOAT8)]));
        c.column_types.insert("comments".into(), columns(&[("id", oid::INT8), ("post_id", oid::INT8), ("body", oid::TEXT)]));
        c.column_types.insert("profiles".into(), columns(&[("user_id", oid::INT8), ("bio", oid::TEXT)]));
        c.functions.insert("random".into());
        c.volatile.insert("random".into());
        c
    }

    #[test]
    fn a_join_on_a_unique_key_of_a_pinned_row_is_at_most_one_row() {
        let mut c = joined();
        // `q` is a comment row of an enclosing query, as a host proving a subquery says.
        c.outer_aliases.insert("q".into(), "comments".into());
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        for sql in [
            // posts by the WHERE, then users through the ON from the post's author.
            "select p.title, u.email from posts p join users u on u.id = p.author_id where p.id = $1",
            "select p.title, u.email from posts p left join users u on u.id = p.author_id where p.id = $1",
            "select p.title, u.email from users u right join posts p on u.id = p.author_id where p.id = $1",
            // A chain, and the same with bare names that only one relation has.
            "select c.body, p.title, u.email from comments c join posts p on p.id = c.post_id join users u on u.id = p.author_id where c.id = $1",
            "select body, title, email from comments c join posts p on p.id = post_id join users u on u.id = author_id where c.id = $1",
            // One-to-one through a key that is also the join column.
            "select u.email, pr.bio from users u left join profiles pr on pr.user_id = u.id where u.id = $1",
            // A self-join, and comma joins with the equalities in the WHERE.
            "select a.email from users a join users b on b.id = a.id where a.id = $1",
            "select p.title, u.email from posts p, users u where u.id = p.author_id and p.id = $1",
            // A derived table and a WITH query of one row each.
            "select s.email from (select * from users where id = $1) s",
            "with one as (select * from users where id = $1) select o.email, p.title from one o join posts p on p.id = o.pinned_post",
            // A LATERAL subquery of one row per row before it, once that row is pinned.
            "select u.email, x.title from users u cross join lateral (select title from posts where author_id = u.id order by id desc limit 1) x where u.id = $1",
            // A column of an enclosing query is a value, as a parameter is.
            "select p.title, u.email from posts p join users u on u.id = p.author_id where p.id = q.post_id",
            // An uncorrelated subquery is a value too, its bare columns its own relation's.
            "select email from users where id = (select author_id from posts where id = $1)",
        ] {
            assert_eq!(at(sql), Ok(()), "{sql}");
        }
    }

    #[test]
    fn a_join_proves_nothing_unless_every_relation_is_pinned() {
        let c = joined();
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        for (sql, fragment) in [
            // Many posts per author.
            ("select p.title from posts p join users u on u.id = p.author_id where u.id = $1", "of `p`"),
            // The ON of a LEFT JOIN pins its right side only, so `u` is any number of rows.
            ("select u.email, p.title from users u left join posts p on p.id = $1", "of `u`"),
            // Nor does it filter its left side: every user comes back, whatever the ON equates.
            ("select u.email, p.title from users u left join posts p on u.id = $1 and p.id = $2", "of `u`"),
            // A FULL JOIN returns a row for each side that matches nothing.
            ("select p.title, u.email from posts p full join users u on u.id = p.author_id where p.id = $1", "FULL JOIN"),
            // USING equates columns the build does not read.
            ("select p.title from posts p join users u using (id) where p.id = $1", "of `u`"),
            // A derived table of many rows, and LATERAL before its outer row is pinned.
            ("select x.title from (select * from posts) x where x.id = $1", "the subquery `x`"),
            ("select u.email, x.title from users u cross join lateral (select title from posts where author_id = u.id limit 1) x", "of `u`"),
            // A function in FROM, and a join with an alias of its own.
            ("select u.email from users u, generate_series(1, 2) g where u.id = $1", "`g` in the FROM"),
            ("select j.email from (users u join posts p on p.author_id = u.id) as j where j.id = $1", "alias of its own"),
            // A column of a relation that is not pinned is no value: each post has its own author.
            ("select p.title, u.email from posts p join users u on u.id = p.author_id where p.author_id = $1", "of `p`"),
        ] {
            refused(at(sql), fragment, sql);
        }
        // Without the column types, a bare name on the other side cannot be placed.
        let bare = "select p.title, u.email from posts p join users u on u.id = author_id where p.id = $1";
        assert!(at_most_one(bare, Dialect::Postgres, &Trust::default(), &catalog()).is_err());
        assert!(at(bare).is_ok());
    }

    #[test]
    fn grouping_and_distinct_over_pinned_values_are_one_row() {
        let c = joined();
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        for sql in [
            "select author_id, count(*) from posts where author_id = $1 group by author_id",
            "select author_id, count(*) from posts where author_id = $1 group by 1",
            "select distinct author_id from posts where author_id = $1",
            "select distinct on (author_id) title from posts where author_id = $1 order by author_id, id desc",
            "select distinct p.author_id, u.email from posts p join users u on u.id = p.author_id where p.author_id = $1",
            // INTERSECT draws from both arms and EXCEPT from its left one.
            "select id from users where id = $1 intersect select author_id from posts",
            "select author_id from posts intersect all select id from users where id = $1",
            "select id from users where id = $1 except select author_id from posts",
        ] {
            assert_eq!(at(sql), Ok(()), "{sql}");
        }
        for (sql, fragment) in [
            ("select title, count(*) from posts where author_id = $1 group by title", "GROUP BY"),
            // The pin's COLLATE may match values the grouping tells apart.
            ("select distinct title from posts where title = $1 collate ci", "does not equate every column"),
            ("select author_id, count(*) from posts where author_id = $1 group by grouping sets ((author_id), ())", "GROUPING SETS"),
            // Some posts meet the user and some its null extension: two groups.
            ("select u.email, count(*) from posts p left join users u on u.id = $1 and u.id = p.author_id group by u.email", "GROUP BY"),
            ("select author_id from posts except select id from users where id = $1", "a set operation"),
            ("select id from users where id = $1 union select author_id from posts", "a set operation"),
        ] {
            refused(at(sql), fragment, sql);
        }
    }

    #[test]
    fn a_unique_index_over_an_expression_or_with_a_predicate_bounds_what_it_covers() {
        let mut c = joined();
        // `create unique index on users (lower(email))`, `… on posts (title) where score is null`,
        // `… on comments (body) where body is not null`, as the catalog reports them.
        c.unique_keys.get_mut("users").expect("users").push(vec!["(lower(email))".into()]);
        c.column_types.get_mut("users").expect("users").insert("(lower(email))".into(), oid::TEXT);
        c.partial_keys.insert("posts".into(), vec![(vec!["title".into()], "score IS NULL".into())]);
        c.partial_keys.insert("comments".into(), vec![(vec!["body".into()], "body IS NOT NULL".into())]);
        c.partial_keys.insert("profiles".into(), vec![(vec!["bio".into()], "bio = 'active'::text".into())]);
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        // The server prints a literal with the cast the statement leaves to the column.
        assert_eq!(at("select user_id from profiles where bio = 'active'"), Ok(()));
        refused(at("select user_id from profiles where bio = $1"), "where bio = 'active'::text", "a parameter is no literal");
        assert_eq!(at("select id from users where lower(email) = lower($1)"), Ok(()));
        assert_eq!(at("select u.id from users u where lower(u.email) = $1"), Ok(()));
        assert_eq!(at("select id from posts where title = $1 and score is null"), Ok(()));
        assert_eq!(at("select p.id from posts p where p.title = $1 and p.score is null"), Ok(()));
        // `col IS NOT NULL` is met by the equality itself, which no NULL passes.
        assert_eq!(at("select id from comments where body = $1"), Ok(()));
        // Another expression, or a read the predicate does not cover.
        refused(at("select id from users where upper(email) = $1"), "does not equate", "another expression");
        refused(at("select id from posts where title = $1"), "where score IS NULL", "no predicate");
        refused(at("select id from posts where title = $1 and (score is null or id = 2)"), "does not equate", "the predicate under an OR");
        // A predicate met only in a join's ON clause does not count.
        refused(at("select p.id from users u join posts p on p.score is null where p.title = $1 and u.id = 1"), "of `p`", "the predicate in an ON");
        // A write reads the same keys.
        assert!(matches!(dml("delete from posts where title = $1 and score is null returning id", &c), Some(Ok(_))));
        assert!(matches!(dml("delete from posts where title = $1 returning id", &c), Some(Err(_))));
    }

    #[test]
    fn a_volatile_value_pins_nothing() {
        let c = joined();
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        // `random()` is evaluated for each row: `id = floor(random() * 3)::int` can match two.
        refused(at("select email from users where id = floor(random() * 3)::int"), "does not equate every column", "random");
        // A function the catalog does not know may be volatile.
        refused(at("select email from users where id = nextval('s')"), "does not equate every column", "nextval");
        refused(at("select author_id, count(*) from posts where author_id = random()::int group by author_id"), "GROUP BY", "group by random");
        assert!(matches!(dml("delete from users where id = random()::int returning id", &c), Some(Err(_))));
        assert_eq!(at("select id from users where email = lower($1)"), Ok(()));
    }

    #[test]
    fn a_value_that_changes_from_row_to_row_pins_nothing() {
        let mut c = joined();
        c.functions.insert("row_number".into());
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        for (sql, fragment) in [
            // A window function, a system column and a whole row read through a field read no
            // column of the FROM by name, and are another value in each row all the same.
            ("select distinct row_number() over () as n from posts", "does not equate every column"),
            ("select distinct ctid::text as c from posts", "does not equate every column"),
            ("select distinct (p).id from posts p", "does not equate every column"),
            ("select distinct on ((p).author_id) title from posts p", "does not equate every column"),
            ("select distinct u.email from posts p join users u on u.id = (p).author_id", "a join returns a row per pair"),
            ("select u.id, count(*) as n from posts p join users u on u.id = (p).author_id group by u.id", "GROUP BY"),
            // An operator the trust check never asked about may call a volatile function.
            ("select email from users where id = (0 +? 0)", "does not equate every column"),
            ("select email from users where id = (0 +? 0)::int8", "does not equate every column"),
        ] {
            refused(at(sql), fragment, sql);
        }
        assert_eq!(at("select id from users where email = 'a' || 'b'"), Ok(()));
        // A built-in spelling a user also defined may resolve to theirs.
        let mut shadowed = joined();
        shadowed.user_operators.insert("||".into());
        assert!(at_most_one("select id from users where email = 'a' || 'b'", Dialect::Postgres, &Trust::default(), &shadowed).is_err());
        // A cast to a type a user's cast function targets may call it for each row; a literal's
        // cast reads the type's input function instead.
        let mut cast = joined();
        cast.cast_targets = Some(HashSet::new());
        assert!(at_most_one("select email from users where id = (1 + 2)::int8", Dialect::Postgres, &Trust::default(), &cast).is_err());
        assert_eq!(at_most_one("select email from users where id = '7'::int8", Dialect::Postgres, &Trust::default(), &cast), Ok(()));
    }

    #[test]
    fn only_a_column_whose_equality_is_identity_is_one_value_for_another_relation() {
        let mut c = catalog();
        c.unique_keys.insert("pv_users".into(), vec![vec!["id".into()]]);
        c.unique_keys.insert("pv_posts".into(), vec![vec!["id".into()]]);
        let columns = |cols: &[(&str, u32)]| cols.iter().map(|(n, t)| (n.to_string(), *t)).collect();
        c.column_types.insert("pv_users".into(), columns(&[("id", oid::INT4), ("email", oid::TEXT)]));
        c.column_types.insert("pv_posts".into(), columns(&[("id", oid::INT4), ("tag", oid::TEXT), ("code", oid::TEXT), ("amount", oid::NUMERIC)]));
        c.nondeterministic.insert("pv_posts".into(), ["tag".to_string()].into_iter().collect());
        c.functions.extend(["ascii".to_string(), "length".to_string()]);
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        // `tag = 'a'` admits 'a' and 'A' under a nondeterministic collation, and `amount = 1`
        // admits 1 and 1.0: equal, yet `ascii` and `::text` tell them apart.
        for sql in [
            "select distinct u.email from pv_posts p join pv_users u on u.id = ascii(p.tag) where p.tag = 'a'",
            "select u.email, count(*) from pv_posts p join pv_users u on u.id = ascii(p.tag) where p.tag = 'a' group by u.email",
            "select distinct u.email from pv_posts p join pv_users u on u.id = length(p.amount::text) where p.amount = 1",
        ] {
            refused(at(sql), "", sql);
        }
        assert_eq!(at("select distinct u.email from pv_posts p join pv_users u on u.id = ascii(p.code) where p.code = 'a'"), Ok(()));
        assert_eq!(at("select distinct u.email from pv_posts p join pv_users u on u.id = p.id * 97 where p.id = 1"), Ok(()));
    }

    #[test]
    fn a_with_query_of_an_enclosing_statement_shadows_the_table_inside_it() {
        let c = joined();
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        // A subquery, a WITH query's body and a set operation's arm are proven on their own, and
        // still read `users` as the WITH query of that name, which has no key.
        for sql in [
            "select t.id from (with users as (select 1::int8 as id union all select 1) select * from (select id from users where id = 1) s) t, users u where u.id = 1",
            "with users as (select 1::int8 as id union all select 1), x as (select id from users where id = 1) select x.id from x",
            "with users as (select 1::int8 as id union all select 1) select id from users where id = 1 intersect all select id from posts",
            "with users as (select 1::int8 as id union all select 1) select s.id from (select id from users where id = 1) s",
        ] {
            refused(at(sql), "", sql);
        }
        assert_eq!(at("with one as (select 1 as n) select s.id from (select id from users where id = 1) s"), Ok(()));
    }

    #[test]
    fn a_column_merged_by_using_and_a_qualified_name_are_neither_sides_key() {
        let mut c = joined();
        c.unique_keys.insert("ua".into(), vec![vec!["id".into()], vec!["k".into()]]);
        c.unique_keys.insert("ub".into(), vec![vec!["id".into()], vec!["k".into()]]);
        let columns = |cols: &[(&str, u32)]| cols.iter().map(|(n, t)| (n.to_string(), *t)).collect();
        c.column_types.insert("ua".into(), columns(&[("id", oid::INT8), ("k", oid::FLOAT8)]));
        c.column_types.insert("ub".into(), columns(&[("id", oid::INT8), ("k", oid::NUMERIC)]));
        let at = |sql: &str| at_most_one(sql, Dialect::Postgres, &Trust::default(), &c);
        // The merged `k` is `ua.k` compared with `ub.k` converted to float8, which maps two
        // numerics to one value.
        refused(at("select ua.id, ub.id from ua join ub using (k) where k = '0.1'"), "a join returns a row per pair", "using");
        refused(at("select ua.id, ub.id from ua natural join ub where k = '0.1'"), "a join returns a row per pair", "natural");
        // A schema-qualified name is the table, never the WITH query of that name.
        refused(at("with users as (select count(*) as n from users) select id from public.users"), "public.users", "with");
        assert!(exactly_one("with users as (select count(*) as n from users) select id from public.users", Dialect::Postgres, &Trust::default(), &c).is_err());
    }

    fn at_most(sql: &str) -> Result<(), String> {
        at_most_one(sql, Dialect::Postgres, &Trust::default(), &catalog())
    }

    #[test]
    fn an_insert_of_one_row_returns_it_unless_something_can_skip_it() {
        let c = catalog();
        assert_eq!(dml("insert into users (email) values ($1) returning id", &c), Some(Ok(Cardinality::One)));
        assert_eq!(dml("insert into users (email) values ($1) on conflict (email) do nothing returning id", &c), Some(Ok(Cardinality::AtMostOne)));
        assert_eq!(dml("insert into users (email) values ($1) on conflict (email) do update set email = excluded.email returning id", &c), Some(Ok(Cardinality::One)));
        // `DO UPDATE … WHERE` leaves a conflicting row alone and returns nothing for it.
        assert_eq!(dml("insert into users (email) values ($1) on conflict (email) do update set email = excluded.email where users.id > 1 returning id", &c), Some(Ok(Cardinality::AtMostOne)));
        // A row trigger that runs BEFORE the insert can return NULL; one on UPDATE matters only
        // where ON CONFLICT turns the insert into an update.
        let mut triggered = catalog();
        triggered.skipping_triggers.insert("users".into(), TRIGGER_ROW_INSERT);
        assert_eq!(dml("insert into users (email) values ($1) returning id", &triggered), Some(Ok(Cardinality::AtMostOne)));
        triggered.skipping_triggers.insert("users".into(), TRIGGER_ROW_UPDATE);
        assert_eq!(dml("insert into users (email) values ($1) returning id", &triggered), Some(Ok(Cardinality::One)));
        assert_eq!(dml("insert into users (email) values ($1) on conflict (email) do update set email = excluded.email returning id", &triggered), Some(Ok(Cardinality::AtMostOne)));
        // A rewrite rule replaces the write with actions of its own, and a view hands it to a
        // table whose triggers and rules the catalog did not read: any number of rows.
        let mut ruled = catalog();
        ruled.rewritten.insert("users".into(), TRIGGER_ROW_INSERT);
        assert!(matches!(dml("insert into users (email) values ($1) returning id", &ruled), Some(Err(_))));
        assert_eq!(dml("delete from users where id = $1 returning id", &ruled), Some(Ok(Cardinality::AtMostOne)));
        ruled.rewritten.insert("users".into(), TRIGGER_ROW_DELETE | TRIGGER_ROW_UPDATE);
        assert!(matches!(dml("delete from users where id = $1 returning id", &ruled), Some(Err(_))));
        assert!(matches!(dml("update users set email = $2 where id = $1 returning id", &ruled), Some(Err(_))));
    }

    #[test]
    fn a_key_holds_only_under_its_own_equality() {
        let mut c = catalog();
        c.column_types.insert("users".into(), [("id".to_string(), oid::INT8), ("email".to_string(), oid::TEXT)].into_iter().collect());
        c.param_types = vec![oid::INT8];
        let at = |sql: &str, c: &Catalog| at_most_one(sql, Dialect::Postgres, &Trust::default(), c);
        assert!(at("select email from users where id = $1", &c).is_ok());
        assert!(at("select email from users where id = 7", &c).is_ok(), "an int4 literal against int8 is integer_ops");
        assert!(at("select id from users where email = 'a@x.io'", &c).is_ok());
        // Converting the column to float8 can make two keys one.
        assert!(at("select email from users where id = $1::float8", &c).is_err());
        c.param_types = vec![oid::FLOAT8];
        assert!(at("select email from users where id = $1", &c).is_err());
        // An explicit COLLATE may compare without telling case apart.
        assert!(at("select id from users where email = 'a@x.io' COLLATE ci", &c).is_err());
        // A value whose type the build cannot name bounds a text key, not an int8 one.
        assert!(at("select id from users where email = lower('A@X.IO')", &c).is_ok());
        assert!(at("select email from users where id = abs(-7)", &c).is_err());
        // With a user's `=` in the database, only the column's own type counts.
        c.equality_shadowed = true;
        assert!(at("select email from users where id = 7", &c).is_err());
        c.param_types = vec![oid::INT8];
        assert!(at("select email from users where id = $1", &c).is_ok());
        // A schema-qualified relation never takes the keys of the one on the search path.
        assert!(at("select name from archive.users a where a.id = $1", &c).is_err());
        // A timestamp key compared with a timestamptz is converted through the session's time
        // zone, where two wall-clock times (02:30 and 03:30 on a day that skips 02:00) are one
        // instant; the other way round, and against a date, the value is converted.
        let mut t = catalog();
        t.unique_keys.insert("ev".into(), vec![vec!["ts".into()], vec!["tstz".into()]]);
        t.column_types.insert("ev".into(), [("ts".to_string(), oid::TIMESTAMP), ("tstz".to_string(), oid::TIMESTAMPTZ)].into_iter().collect());
        assert!(at("select 1 from ev where ts = '2026-03-08 03:30'::timestamptz", &t).is_err());
        assert!(at("select 1 from ev where tstz = '2026-03-08 03:30'::timestamp", &t).is_ok());
        assert!(at("select 1 from ev where ts = '2026-03-08'::date", &t).is_ok());
    }

    #[test]
    fn a_limit_with_ties_bounds_nothing() {
        assert!(at_most("select id from users order by email fetch first 1 rows only").is_ok());
        assert!(at_most("select id from users order by email limit 1").is_ok());
        // Every row that ties with the first comes along.
        assert!(at_most("select id from users order by email fetch first 1 rows with ties").is_err());
    }

    #[test]
    fn grouped_by_a_pinned_relation_is_one_group() {
        assert!(at_most("select u.id, count(p.id) from users u left join posts p on p.user_id = u.id where u.id = $1 group by u.id").is_ok());
        assert!(at_most("select u.id, u.name, count(*) from users u join posts p on p.user_id = u.id where u.id = $1 and p.title = $2 group by u.id, u.name having count(*) > 1").is_ok());
        assert!(at_most("select id, count(*) from users where email = $1 group by id").is_ok());
        // A bare column that only `users` has is its column: had `posts` it too, the server would
        // have refused the name as ambiguous.
        assert!(at_most("select u.id, count(*) from users u join posts p on p.user_id = u.id where email = $1 group by email").is_ok());
    }

    #[test]
    fn grouped_by_another_relation_or_pinned_to_a_column_is_many() {
        assert!(at_most("select u.id, p.title, count(*) from users u join posts p on p.user_id = u.id where u.id = $1 group by u.id, p.title").is_err());
        assert!(at_most("select u.id, count(*) from users u join posts p on true where u.id = p.user_id group by u.id").is_err());
        assert!(at_most("select u.id, count(*) from users u join posts p on p.user_id = u.id where u.name = $1 group by u.id").is_err());
        assert!(at_most("select u.id, count(*) from users u join posts p on p.user_id = u.id where u.id = $1 group by title").is_err());
        assert!(at_most("select u.id, count(*) from users u join posts p on p.user_id = u.id where u.id = $1 group by u.id union all select 1, 2").is_err());
    }


    /// A stock Postgres catalog for the names these tests call: `generate_series` and `unnest`
    /// return sets, the rest do not. `todos` has a primary key and `org_members` a two-column one.
    fn stock() -> Catalog {
        let set = |names: &[&str]| names.iter().map(|n| n.to_string()).collect::<HashSet<String>>();
        let key = |cols: &[&str]| vec![cols.iter().map(|c| c.to_string()).collect::<Vec<String>>()];
        Catalog {
            functions: set(&["generate_series", "unnest", "count", "sum", "max", "abs", "lower"]),
            set_returning: set(&["generate_series", "unnest"]),
            set_returning_operators: HashSet::new(),
            unique_keys: [
                ("todos".to_string(), key(&["id"])),
                ("org_members".to_string(), key(&["org_id", "user_id"])),
                ("users".to_string(), vec![vec!["id".to_string()], vec!["email".to_string()]]),
            ]
            .into_iter()
            .collect(),
            ..Catalog::default()
        }
    }

    fn pg(sql: &str) -> Result<(), String> {
        exactly_one(sql, Dialect::Postgres, &Trust::default(), &stock())
    }

    fn lite(sql: &str) -> Result<(), String> {
        exactly_one(sql, Dialect::Sqlite, &Trust::default(), &Catalog::default())
    }

    fn refused(result: Result<(), String>, fragment: &str, sql: &str) {
        match result {
            Ok(()) => panic!("{sql}: proven, expected a refusal naming {fragment:?}"),
            Err(why) => assert!(why.contains(fragment), "{sql}: {why:?} does not name {fragment:?}"),
        }
    }

    #[test]
    fn the_canonical_gate_and_augmenter_are_exactly_one_row_on_both_dialects() {
        let gate = "SELECT EXISTS (SELECT 1 FROM org_members WHERE org_id = $1 AND user_id = $2) AS authorized, (SELECT role FROM org_members WHERE org_id = $1 AND user_id = $2) AS role";
        assert_eq!(pg(gate), Ok(()));
        assert_eq!(lite(&gate.replace('$', "?")), Ok(()));
        let aggregate = "SELECT count(*) > 0 AS authorized FROM org_members WHERE org_id = $1 AND user_id = $2";
        assert_eq!(pg(aggregate), Ok(()));
        assert_eq!(lite(&aggregate.replace('$', "?")), Ok(()));
        for sql in [
            "SELECT $1::text AS caller, lower($2) AS name",
            "SELECT coalesce((SELECT role = 'admin' FROM org_members WHERE user_id = $1), false) AS authorized",
            // A set-returning function inside a subquery answers one value to this level.
            "SELECT EXISTS (SELECT generate_series(1, 3)) AS authorized",
            // A WITH is fine, whatever the CTE returns.
            "WITH many AS (SELECT generate_series(1, 3) AS g) SELECT EXISTS (SELECT 1 FROM many) AS authorized",
            // An ungrouped aggregate is one row over no input, with or without a FROM.
            "SELECT count(*) AS n FROM pg_class WHERE false",
            "SELECT count(*) AS n WHERE false",
            "SELECT DISTINCT count(*) AS n FROM pg_class",
            "SELECT count(*) FILTER (WHERE false) AS n FROM pg_class",
            // The windowed sum runs over the aggregate query's one row.
            "SELECT sum(count(*)) OVER () AS n FROM pg_class",
        ] {
            assert_eq!(pg(sql), Ok(()), "{sql}");
        }
        // SQLite has no set-returning function to look up, so it needs no catalog.
        assert_eq!(lite("SELECT lower(?1) AS name, EXISTS (SELECT 1 FROM t WHERE id = ?2) AS authorized"), Ok(()));
        assert_eq!(lite("SELECT max(amount) AS top FROM t"), Ok(()));
    }

    #[test]
    fn a_where_with_no_from_can_remove_the_row() {
        refused(pg("SELECT 1 AS authorized WHERE $1 = 1"), "a WHERE with no FROM can remove the row", "pg");
        refused(lite("SELECT 1 AS authorized WHERE ?1 = 1"), "a WHERE with no FROM can remove the row", "sqlite");
    }

    #[test]
    fn a_set_returning_call_at_this_level_multiplies_or_removes_the_row() {
        for sql in [
            "SELECT generate_series(1, 3) AS g",
            "SELECT unnest($1::int[]) AS u",
            "SELECT abs(generate_series(-1, 1)) AS nested",
            "SELECT pg_catalog.generate_series(1, 3) AS qualified",
            "SELECT generate_series(1, 2) IS NOT NULL AS under_a_null_test",
            "SELECT NULLIF(1, abs(generate_series(1, 2))) AS in_nullif_argument",
            "SELECT 1 AS x ORDER BY generate_series(1, 2)",
            "SELECT DISTINCT ON (generate_series(1, 2)) 1 AS x",
            "SELECT $1 IN (SELECT 1) AND generate_series(1, 2) > 0 AS x",
            // An aggregate beside one is as many rows as the set.
            "SELECT count(*) AS n, generate_series(1, 3) AS g FROM pg_class WHERE false",
        ] {
            refused(pg(sql), "set-returning function", sql);
        }
        refused(pg("SELECT mystery($1) AS authorized"), "is not in the catalog", "mystery");
        // With no catalog at all, every Postgres call is unproven, and a call-free one is not.
        refused(exactly_one("SELECT lower($1) AS x", Dialect::Postgres, &Trust::default(), &Catalog::default()), "not in the catalog", "empty");
        assert_eq!(exactly_one("SELECT $1 = 'a' AS x", Dialect::Postgres, &Trust::default(), &Catalog::default()), Ok(()));
    }

    #[test]
    fn an_operator_over_a_set_returning_function_is_a_set_returning_call() {
        let mut catalog = stock();
        let with = |c: &Catalog, sql: &str| exactly_one(sql, Dialect::Postgres, &Trust::default(), c);
        assert_eq!(with(&catalog, "SELECT 1 ##> 3 AS x"), Ok(()), "an operator no set-returning function implements is a value");
        catalog.set_returning_operators.insert("##>".into());
        refused(with(&catalog, "SELECT 1 ##> 3 AS x"), "operator ##>", "##>");
        refused(with(&catalog, "SELECT abs(1 ##> 3) IS NOT NULL AS x"), "operator ##>", "nested ##>");
        assert_eq!(with(&catalog, "SELECT 1 + 1 AS x"), Ok(()));
        catalog.set_returning_operators = ["<="].map(String::from).into_iter().collect();
        refused(with(&catalog, "SELECT $1 BETWEEN 1 AND 2 AS x"), "operator <=", "between");
        catalog.set_returning_operators = ["="].map(String::from).into_iter().collect();
        refused(with(&catalog, "SELECT $1 IN (SELECT 1) AS x"), "operator =", "in subquery");
        refused(with(&catalog, "SELECT $1 IS DISTINCT FROM 2 AS x"), "operator =", "distinct");
    }

    #[test]
    fn limits_group_by_having_and_set_operations_are_unproven() {
        for (sql, fragment) in [
            ("SELECT 1 AS x LIMIT 0", "LIMIT or an OFFSET"),
            ("SELECT 1 AS x LIMIT 1", "LIMIT or an OFFSET"),
            ("SELECT 1 AS x OFFSET 1", "LIMIT or an OFFSET"),
            ("SELECT 1 AS x FETCH FIRST 0 ROWS ONLY", "LIMIT or an OFFSET"),
            ("SELECT count(*) AS n FROM pg_class GROUP BY relkind", "GROUP BY"),
            ("SELECT 1 AS x GROUP BY 1", "GROUP BY"),
            ("SELECT count(*) AS n FROM pg_class HAVING count(*) > 1", "HAVING"),
            ("SELECT true AS authorized UNION SELECT false", "set operation"),
            ("SELECT 1 AS x EXCEPT SELECT 1", "set operation"),
            ("VALUES (true)", "VALUES"),
            ("INSERT INTO todos (title) VALUES ($1) RETURNING id", "only a SELECT"),
            ("SELECT 1; SELECT 2", "not exactly one statement"),
        ] {
            refused(pg(sql), fragment, sql);
            refused(lite(&sql.replace('$', "?")), fragment, sql);
        }
        // A window call is not an aggregate: a row per input row.
        refused(pg("SELECT count(*) OVER () AS n FROM pg_class"), "a FROM returns a row per row", "window");
        refused(pg("SELECT role FROM org_members WHERE user_id = $1"), "a FROM returns a row per row", "plain read");
        // SQLite's two-argument max is a scalar function, not an aggregate.
        refused(lite("SELECT max(a, b) AS top FROM t"), "a FROM returns a row per row", "scalar max");
        // An aggregate name somebody redefined is not the aggregate.
        let shadowed = Trust { shadowed: ["count".to_string()].into_iter().collect(), cast_targets: None, ..Default::default() };
        refused(exactly_one("SELECT count(*) AS n FROM pg_class", Dialect::Postgres, &shadowed, &stock()), "a FROM returns a row per row", "shadowed count");
        // SQLite syntax outside the shared grammar proves nothing.
        refused(lite("SELECT 1 AS x LIMIT 5, 0"), "syntax outside it", "sqlite limit");
        refused(pg("SELECT FROM WHERE"), "does not parse", "garbage");
    }

    fn pg_most(sql: &str) -> Result<(), String> {
        at_most_one(sql, Dialect::Postgres, &Trust::default(), &stock())
    }

    #[test]
    fn a_unique_key_every_column_of_which_is_equated_bounds_the_read_to_one_row() {
        for sql in [
            "SELECT title FROM todos WHERE id = $1",
            "SELECT title FROM todos t WHERE t.id = $1",
            // The key's columns in either order, beside anything else the WHERE asks for.
            "SELECT role FROM org_members WHERE user_id = $1 AND org_id = $2 AND role <> 'banned'",
            "SELECT role FROM org_members m WHERE m.org_id = $1 AND m.user_id = m2.user_id",
            // Either of two unique keys will do.
            "SELECT id FROM users WHERE email = $1",
            // An equality against a correlated column of an enclosing query, which is the
            // one-to-one nesting case.
            "SELECT u.id, u.email FROM users u WHERE u.id = p.author_id",
            // An OFFSET only ever removes rows.
            "SELECT title FROM todos WHERE id = $1 OFFSET 2",
            // A constant LIMIT caps anything, key or no key, set operation included.
            "SELECT title FROM todos WHERE owner_id = $1 ORDER BY created_at DESC LIMIT 1",
            "SELECT 1 AS x UNION SELECT 2 LIMIT 1",
            "SELECT generate_series(1, 3) AS g LIMIT 1",
            "SELECT title FROM todos LIMIT 0",
            // A join on the key of a row the WHERE pins.
            "SELECT t.title FROM todos t JOIN users u ON u.id = t.owner_id WHERE t.id = $1",
            // Exactly one row is at most one row.
            "SELECT count(*) AS n FROM todos",
            "SELECT $1::text AS caller",
        ] {
            assert_eq!(pg_most(sql), Ok(()), "{sql}");
        }
    }

    #[test]
    fn a_read_the_key_does_not_cover_is_not_bounded_to_one_row() {
        for (sql, fragment) in [
            // Half of a two-column key.
            ("SELECT role FROM org_members WHERE org_id = $1", "does not equate every column"),
            // Not an equality: a range, a pattern, `IS NOT DISTINCT FROM`, `= ANY`.
            ("SELECT title FROM todos WHERE id > $1", "does not equate every column"),
            ("SELECT title FROM todos WHERE id IS NOT DISTINCT FROM $1", "does not equate every column"),
            ("SELECT title FROM todos WHERE id = ANY($1)", "does not equate every column"),
            // An OR beside the equality widens what matches.
            ("SELECT title FROM todos WHERE id = $1 OR owner_id = $2", "does not equate every column"),
            // The other side is a column of the same relation, so it restricts nothing.
            ("SELECT title FROM todos WHERE id = owner_id", "does not equate every column"),
            ("SELECT title FROM todos t WHERE t.id = t.owner_id", "does not equate every column"),
            // No WHERE at all.
            ("SELECT title FROM todos", "does not equate every column"),
            // A table with no unique index the build can read.
            ("SELECT body FROM events WHERE id = $1", "no unique index"),
            // A WITH query of the same name is not the table.
            ("WITH todos AS (SELECT id FROM events) SELECT id FROM todos WHERE id = $1", "the WITH query `todos`"),
            // A derived table of any number of rows, and a UNION.
            ("SELECT x.title FROM (SELECT * FROM todos) x WHERE x.id = $1", "the subquery `x`"),
            ("SELECT title FROM todos WHERE id = $1 UNION SELECT title FROM todos", "a set operation"),
            // A LIMIT above one, or one the build cannot read, bounds nothing.
            ("SELECT title FROM todos WHERE id = $1 LIMIT 2", "bounds nothing"),
            ("SELECT title FROM todos WHERE id = $1 LIMIT $2", "bounds nothing"),
            // A set-returning call multiplies the one row the key found.
            ("SELECT generate_series(1, 3) AS g FROM todos WHERE id = $1", "set-returning function"),
        ] {
            refused(pg_most(sql), fragment, sql);
        }
        // The key pins one row of `todos` and the GROUP BY reads only its columns: one group at most.
        assert_eq!(pg_most("SELECT count(*) AS n FROM todos WHERE id = $1 GROUP BY owner_id"), Ok(()));
    }

    #[test]
    fn a_select_over_one_with_query_returns_that_querys_rows_at_most() {
        // One row in, one row out; a WHERE over it can only remove the row.
        assert_eq!(pg("WITH one AS (SELECT 1 AS id) SELECT id FROM one"), Ok(()));
        refused(pg("WITH one AS (SELECT 1 AS id) SELECT id FROM one WHERE id = $1"), "can remove its row", "a WHERE over a one-row WITH");
        assert_eq!(pg_most("WITH one AS (SELECT 1 AS id) SELECT id FROM one WHERE id = $1"), Ok(()));
        // A one-row INSERT with RETURNING, and a write by key.
        assert_eq!(pg("WITH ins AS (INSERT INTO todos (title) VALUES ($1) RETURNING id) SELECT id FROM ins"), Ok(()));
        assert_eq!(pg_most("WITH gone AS (DELETE FROM todos WHERE id = $1 RETURNING id) SELECT id FROM gone"), Ok(()));
        refused(pg_most("WITH gone AS (DELETE FROM todos WHERE owner_id = $1 RETURNING id) SELECT id FROM gone"), "the WITH query `gone`", "a write that is not by key");
        // A key read through the WITH query.
        assert_eq!(pg_most("WITH mine AS (SELECT * FROM todos WHERE id = $1) SELECT title FROM mine"), Ok(()));
        // The GROUP BY proof over a joined relation still refuses a column of the other side.
        refused(pg_most("SELECT t.title, count(*) AS n FROM users u JOIN todos t ON t.owner_id = u.id WHERE u.id = $1 GROUP BY t.title"), "GROUP BY", "a GROUP BY on the joined relation");
    }

    #[test]
    fn the_same_proof_runs_on_sqlite_against_the_keys_sqlite_reports() {
        let catalog = Catalog { unique_keys: stock().unique_keys, ..Catalog::default() };
        let lite = |sql: &str| at_most_one(sql, Dialect::Sqlite, &Trust::default(), &catalog);
        assert_eq!(lite("SELECT title FROM todos WHERE id = ?1"), Ok(()));
        assert_eq!(lite("SELECT title FROM todos WHERE owner_id = ?1 LIMIT 1"), Ok(()));
        refused(lite("SELECT title FROM todos WHERE owner_id = ?1"), "does not equate every column", "sqlite");
        // With no key read at all, nothing is proven — which is the fail-closed direction.
        refused(
            at_most_one("SELECT title FROM todos WHERE id = ?1", Dialect::Sqlite, &Trust::default(), &Catalog::default()),
            "no unique index",
            "empty catalog",
        );
    }

    #[test]
    fn only_a_constant_limit_zero_proves_there_is_no_row() {
        for sql in [
            "SELECT 1 AS x LIMIT 0",
            "SELECT 1 AS x FETCH FIRST 0 ROWS ONLY",
            "SELECT id FROM todos WHERE owner_id = $1 LIMIT 0",
            "SELECT 1 AS x UNION SELECT 2 LIMIT 0",
        ] {
            assert_eq!(never_a_row(sql, Dialect::Postgres), Some("LIMIT 0 returns no rows"), "{sql}");
        }
        assert_eq!(never_a_row("SELECT id FROM todos WHERE owner_id = ?1 LIMIT 0 OFFSET ?2", Dialect::Sqlite), Some("LIMIT 0 returns no rows"));
        for sql in ["SELECT 1 AS x", "SELECT 1 AS x LIMIT 1", "SELECT 1 AS x LIMIT $1", "SELECT 1 AS x LIMIT ALL", "SELECT 1 AS x OFFSET 1", "SELECT 1 AS x WHERE false"] {
            assert_eq!(never_a_row(sql, Dialect::Postgres), None, "{sql}");
        }
    }
}
