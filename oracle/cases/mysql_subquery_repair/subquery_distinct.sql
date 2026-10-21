select u.id, (select count(distinct p.tag) from posts p where p.user_id = u.id) as tags from users u
