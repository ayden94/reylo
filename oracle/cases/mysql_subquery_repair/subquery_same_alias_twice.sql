select u.id as n, (select count(*) from posts p where p.user_id = u.id) as n from users u
