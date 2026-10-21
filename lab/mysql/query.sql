select id, name, email, big, n, at, j, flag, raw, id + 1 as next_id
from users
where id = :id and name <> :skip
