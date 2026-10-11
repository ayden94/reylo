select u.id, (select any_value(p.title) from posts p where p.user_id = u.id) as some from users u
