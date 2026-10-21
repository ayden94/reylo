select u.id, (select count(*) from posts p where p.user_id = u.id) + 1 as n, coalesce((select max(p.id) from posts p where p.user_id = u.id), 0) as last from users u
