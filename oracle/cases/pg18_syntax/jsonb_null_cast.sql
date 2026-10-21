select j::int4 as i, j::numeric as n, j::bool as b, j::float8 as f, cast(j as int8) as l, j::int2 as h, j::float4 as r, j::text as t from docs where jsonb_typeof(j) = 'null'
