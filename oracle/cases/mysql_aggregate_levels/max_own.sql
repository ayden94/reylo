select u.id, (select max(p.id) from posts p where p.user_id = u.id) as last from users u
