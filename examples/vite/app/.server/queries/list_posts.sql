select p.id, p.title, p.body, u.id as user_id, u.name as author
from posts p
join users u on u.id = p.user_id
order by p.id
