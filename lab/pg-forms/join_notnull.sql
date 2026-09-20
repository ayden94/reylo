select u.id, u.name, p.id as post_id, p.title, p.views
from pgf_users u left join pgf_posts p on p.user_id = u.id
where p.id is not null
order by u.id, p.id
