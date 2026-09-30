select p.id, p.title, u.name as author, u.email
from porg_posts p
left join porg_users u on u.id = p.user_id
