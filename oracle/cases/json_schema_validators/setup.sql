-- The Postgres JSON Schema validators besides pg_jsonschema (json_schema_checks), each read as
-- it was measured (docs/json-schema-checks/postgres/): the engine knows a validator by the
-- extension that defines the function a CHECK calls, never by the function's name, since
-- theory's (formerly Tembo's) `jsonschema` defines pg_jsonschema's very names, and a function of
-- that name from no extension proves nothing. scripts/gates.sh installs them all.
create schema boon;
create extension jsonschema schema boon;
create extension "postgres-json-schema";
create extension is_jsonb_valid;
create schema adunstan;
create extension json_schema_validate schema adunstan;
-- theory's jsonschema: a schema with no `$schema` is read under whichever draft a session's
-- jsonschema.default_draft names, so `const`, which draft 4 lacks, narrows nothing there; with
-- one it is read under that draft. jsonschema_validates takes the value first.
create table boon_docs (
  id int primary key,
  a jsonb check (boon.jsonb_matches_schema('{"type": "object", "properties": {"k": {"const": "x"}}, "required": ["k"]}', a)),
  b jsonb check (boon.jsonb_matches_schema('{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object", "properties": {"k": {"const": "x"}}, "required": ["k"]}', b)),
  c jsonb check (boon.jsonschema_validates(c, '{"type": "array", "items": {"type": "integer"}}'))
);
-- A session that reads drafts as draft 4 stores what the default session refuses.
set jsonschema.default_draft = 'V4';
insert into boon_docs values (1, '{"k": "y"}', '{"k": "x", "more": 1}', '[1, 2]');
reset jsonschema.default_draft;
-- gavinwahl's: draft 4, only `#/` pointers resolved.
create table gw (id int primary key, d jsonb check (validate_json_schema('{"type": "object", "properties": {"n": {"type": "integer"}}, "required": ["n"]}', d)));
insert into gw values (1, '{"n": 1, "x": []}');
-- is_jsonb_valid: one function for draft 4, one for draft 7.
create table ijv (id int primary key, d4 jsonb check (is_jsonb_valid('{"type": "string", "const": "a"}', d4)), d7 jsonb check (is_jsonb_valid_draft_v7('{"const": "a"}', d7)));
insert into ijv values (1, '"b"', '"a"');
-- adunstan's: the value first, no tuples, no patternProperties.
create table adu (
  id int primary key,
  d jsonb check (adunstan.jsonschema_is_valid(d, '{"type": "array", "items": [{"type": "string"}]}'::jsonb)),
  e jsonb check (adunstan.jsonschema_is_valid(e, '{"type": "object", "patternProperties": {"^x": {"type": "string"}}, "additionalProperties": false}'::jsonb))
);
insert into adu values (1, '[1, 2]', '{"xa": 1}');
create function public.jsonb_matches_schema(schema json, instance jsonb) returns boolean language sql immutable strict as $$ select true $$;
create table fake (id int primary key, d jsonb check (public.jsonb_matches_schema('{"type": "string"}', d)));
insert into fake values (1, '1');
