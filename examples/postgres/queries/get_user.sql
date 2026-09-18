-- @returns maybeOne
select id, name, email, role, created_at
from users
where id = :id
