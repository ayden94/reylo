select pgf_greet(u.name) as greeting,
  pgf_greet_strict(u.email) as strict_greeting,
  pgf_double(u.id) as doubled,
  coalesce(u.email, 'none') as email_or,
  nullif(u.name, 'ada') as not_ada,
  now() as ts,
  gen_random_uuid() as fresh,
  lower(u.email) as lowered
from pgf_users u
where u.id = :id
