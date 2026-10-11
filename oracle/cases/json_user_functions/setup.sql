-- A JSON constructor is read as one only where its name can be nothing but the built-in: a call
-- qualified with a schema is that schema's function, and an unqualified one may resolve to a
-- function a user defined under the name, which matches `(text, text)` better than the built-in's
-- variadic arguments do. `json_build_array` has no such twin here.
create table users (id bigint primary key, name text not null);
create schema app;
create function app.json_build_object(k text, v text) returns json language sql as $$ select null::json $$;
create function public.jsonb_build_object(k text, v text) returns jsonb language sql as $$ select null::jsonb $$;
create function public.to_jsonb(u users) returns jsonb language sql as $$ select null::jsonb $$;
create function app.json_agg(v text) returns json language sql as $$ select null::json $$;
