select u.id, (select sum(p.id) from posts p where p.user_id = u.id) as total from users u
