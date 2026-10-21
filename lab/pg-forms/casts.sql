select u.id::text as id_text,
  u.name::varchar(10) as short_name,
  u.score::int as score_int,
  cast(u.big as numeric) as big_num,
  '2024-01-01'::date as day,
  :since::timestamptz as since,
  (u.meta ->> 'k')::int as k,
  null::int as nothing
from pgf_users u
where u.id = :id
