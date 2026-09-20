select u.id, jsonb_path_query(coalesce(u.meta, '{}'), '$.k') as k
from pgf_users u
where u.id = :id
