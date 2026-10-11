select (j -> 'a')::int4 as a, j::text as t from docs where jsonb_typeof(j) = 'object'
