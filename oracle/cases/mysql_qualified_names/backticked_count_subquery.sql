select u.id, (select `count`(p.id) from posts p where p.user_id = u.id) as n from users u
