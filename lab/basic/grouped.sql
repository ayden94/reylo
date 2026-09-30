select u.id, u.name, count(p.id) as posts, array[u.id] as ids
from porg_users u left join porg_posts p on p.user_id = u.id
where u.id = :id
group by u.id
