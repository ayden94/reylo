select u.id, u.name, p.id as post_id, p.title, p.views
from pgf_users u full join pgf_posts p on p.user_id = u.id
order by u.id, p.id
