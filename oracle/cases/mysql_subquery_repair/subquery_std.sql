select u.id, (select std(p.id) from posts p where p.user_id = u.id) as spread from users u
