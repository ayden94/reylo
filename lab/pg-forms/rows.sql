select row(u.id, u.name) as pair_anon,
  (u.id, u.name)::pgf_pair as pair_typed,
  to_jsonb(u) as whole_json,
  u as whole_row
from pgf_users u
where u.id = :id
