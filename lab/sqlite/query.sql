-- @returns maybeOne
-- @param id number
select id, name, email, score, avatar, created_at, id + 1 as next_id
from users
where id = :id
