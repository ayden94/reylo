select u.id, u.name, (select count(*) from posts p where p.user_id = u.id) as n, (select max(p.title) from posts p where p.user_id = u.id) as last from users u
