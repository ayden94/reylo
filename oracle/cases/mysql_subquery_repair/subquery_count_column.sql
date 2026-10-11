select u.id, (select count(p.tag) from posts p where p.user_id = u.id) as tagged from users u
