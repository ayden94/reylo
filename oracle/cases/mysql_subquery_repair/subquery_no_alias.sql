select u.id, (select count(*) from posts p where p.user_id = u.id) from users u
