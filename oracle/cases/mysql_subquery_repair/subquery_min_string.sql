select u.id, (select min(p.title) from posts p where p.user_id = u.id) as first from users u
