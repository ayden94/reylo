select p.id, p.title, p.body, u.name as author
from posts p
join users u on u.id = p.user_id
where u.role = :role
order by p.id
