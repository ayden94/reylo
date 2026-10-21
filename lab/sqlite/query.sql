select id, name, email, score, avatar, created_at, id + 1 as next_id, count(*) over () as total
from users
where id = :id and name like :pattern
