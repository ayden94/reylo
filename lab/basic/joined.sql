select p.id, p.title, u.name as author, u.email
from lando_posts p
left join lando_users u on u.id = p.user_id
