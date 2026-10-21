select u.id, (select coalesce(sum(p.id), 0) from posts p where p.user_id = u.id) as total from users u
