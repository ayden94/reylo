select max(u.id) as top, (select count(*) from posts p where p.user_id = max(u.id)) as n from users u
