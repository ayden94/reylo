select u.id, u.name from users u where (select count(*) from posts p where p.user_id = u.id) > 0
