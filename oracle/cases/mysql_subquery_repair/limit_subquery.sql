select u.id, (select p.title from posts p where p.user_id = u.id order by p.id limit 1) as first from users u
