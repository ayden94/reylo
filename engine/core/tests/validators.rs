//! Every value a database's JSON Schema validator accepted, in the measurements of
//! `wiki/json-schema-checks/`, fits the type the engine reads off the same schema for that
//! validator: the soundness of `schema_type`, checked against what the validators did rather
//! than what the drafts say.

use reylo_engine_core::schema_type::{admits, type_schema_any, Validator, ADUNSTAN, BOON, GAVINWAHL, IS_JSONB_VALID_4, IS_JSONB_VALID_7, JSONSCHEMA_0_17, JSONSCHEMA_0_37, MYSQL, PG_JSONSCHEMA, SQLITE_JSONSCHEMA};
use serde_json::{Map, Value};
use std::path::PathBuf;

fn checks(path: &str) -> Value {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../wiki/json-schema-checks").join(path);
    serde_json::from_str(&std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()))).unwrap()
}

/// The probes of a probe file by id, with the drafts it names (none for a file that runs the
/// drafts of `probes.json`).
fn probes(path: &str) -> (Map<String, Value>, Map<String, Value>) {
    let file = checks(path);
    let by_id = file["probes"].as_array().unwrap().iter().map(|p| (p["id"].as_str().unwrap().to_string(), p.clone())).collect();
    (by_id, file["drafts"].as_object().cloned().unwrap_or_default())
}

/// The schema a probe ran under a draft, and its instance; `None` where either is no JSON the
/// engine reads (`1e400`) or no schema at all.
fn case(probe: &Value, draft: &Value) -> Option<(Value, Value)> {
    let mut schema = match probe.get("schema_text") {
        Some(text) => serde_json::from_str(text.as_str()?).ok()?,
        None => probe["schema"].clone(),
    };
    if let (Some(uri), Some(map)) = (draft.as_str(), schema.as_object_mut()) {
        map.insert("$schema".into(), Value::String(uri.to_string()));
    }
    let instance = match probe.get("instance_text") {
        Some(text) => serde_json::from_str(text.as_str()?).ok()?,
        None => probe.get("instance")?.clone(),
    };
    Some((schema, instance))
}

/// Each accepted value that does not fit, as `probe/draft: why`.
fn unsound(validators: &[&Validator], outcomes: &Map<String, Value>, probes: &Map<String, Value>, drafts: &Map<String, Value>, outcome: impl Fn(&Value) -> Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let mut checked = 0;
    for (id, by_draft) in outcomes {
        let Some(probe) = probes.get(id) else { continue };
        for (draft, result) in by_draft.as_object().unwrap() {
            if outcome(result) != Some("valid") {
                continue;
            }
            let Some((schema, instance)) = case(probe, &drafts[draft]) else { continue };
            checked += 1;
            let ty = type_schema_any(&schema, validators);
            if !admits(&ty, &instance) {
                out.push(format!("{id}/{draft}: {schema} accepted {instance}, typed {ty}"));
            }
        }
    }
    assert!(checked > 50, "only {checked} accepted values were checked");
    out
}

#[test]
fn sqlite_jsonschema() {
    let (probes, drafts) = probes("probes.json");
    let matrix = checks("sqlite/out/matrix.json");
    let outcomes = matrix.as_object().unwrap().iter().find(|(k, _)| k.starts_with("sqlite-jsonschema")).unwrap().1.as_object().unwrap();
    let bad = unsound(&[&SQLITE_JSONSCHEMA], outcomes, &probes, &drafts, |r| r.as_str());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn mysql_json_schema_valid() {
    let (mut probes, drafts) = probes("probes.json");
    let (extra, _) = self::probes("mysql/extra-probes.json");
    probes.extend(extra);
    let matrix = checks("mysql/out/matrix.json");
    for (server, measured) in matrix.as_object().unwrap().iter().filter(|(k, _)| k.starts_with("mysql")) {
        // What a CHECK on a JSON column did; the function's own answer where no CHECK ran.
        let bad = unsound(&[&MYSQL], measured["probes"].as_object().unwrap(), &probes, &drafts, |r| r.get("check").or_else(|| r.get("json")).and_then(Value::as_str));
        assert!(bad.is_empty(), "{server}:\n{}", bad.join("\n"));
    }
}

#[test]
fn reading_mysql_by_the_specification_would_be_unsound() {
    // The check has teeth: a profile that reads every schema as draft 2020-12, references and
    // all, types values MySQL accepted (`#/$defs/s` it leaves unresolved, `const` it ignores).
    let (mut probes, drafts) = probes("probes.json");
    probes.extend(self::probes("mysql/extra-probes.json").0);
    let naive = Validator { fallback: &[reylo_engine_core::schema_type::Draft::Twenty], dollar_pointers: true, openapi_roots: false, ..BOON };
    let matrix = checks("mysql/out/matrix.json");
    let bad = unsound(&[&naive], matrix["mysql-8.4"]["probes"].as_object().unwrap(), &probes, &drafts, |r| r.get("check").and_then(Value::as_str));
    assert!(bad.iter().any(|b| b.starts_with("r2/")) && bad.iter().any(|b| b.starts_with("e2/")), "{}", bad.join("\n"));
}

/// Each Postgres validator measured, with the profiles the engine reads its CHECKs by: every
/// release of pg_jsonschema, the unreleased main branch included, against the union the engine
/// reads any version by, and each crate line against its own profile.
fn postgres_profile(name: &str) -> Option<&'static [&'static Validator]> {
    Some(match name {
        n if n.starts_with("crate jsonschema 0.16.1+drafts") => return None,
        n if n.starts_with("crate jsonschema 0.16") || n.starts_with("crate jsonschema 0.17") => &[&JSONSCHEMA_0_17],
        n if n.starts_with("crate jsonschema 0.37") => &[&JSONSCHEMA_0_37],
        n if n.starts_with("crate jsonschema 0.58") || n.starts_with("pg_jsonschema") => {
            // `jsonb_matches_compiled_schema` takes a type of the unreleased branch's own, which
            // no CHECK the engine reads calls.
            if n.contains("compiled") {
                return None;
            }
            PG_JSONSCHEMA
        }
        n if n.starts_with("jsonschema (boon)") => &[&BOON],
        n if n.starts_with("postgres-json-schema") => &[&GAVINWAHL],
        n if n.starts_with("is_jsonb_valid 0.1.4 is_jsonb_valid_draft_v7") => &[&IS_JSONB_VALID_7],
        n if n.starts_with("is_jsonb_valid") => &[&IS_JSONB_VALID_4],
        n if n.starts_with("json_schema_validate") => &[&ADUNSTAN],
        _ => return None,
    })
}

#[test]
fn postgres_validators() {
    let (mut probes, drafts) = probes("probes.json");
    probes.extend(self::probes("postgres/extra-probes.json").0);
    let matrix = checks("postgres/out/matrix.json");
    let mut read = 0;
    for (name, outcomes) in matrix.as_object().unwrap() {
        let Some(profile) = postgres_profile(name) else { continue };
        read += 1;
        let bad = unsound(profile, outcomes.as_object().unwrap(), &probes, &drafts, |r| r.as_str());
        assert!(bad.is_empty(), "{name}:\n{}", bad.join("\n"));
    }
    assert!(read >= 20, "only {read} validators were read");
}
