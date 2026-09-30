-- @returns maybeOne
select id, name, email, role, tags, created_at, big, id + 1 as next_id
from porg_users
where id = :id and name = :name
