select u.id, (select count(*) from posts p where false) as n, (select max(p.id) from posts p where false) as m from users u
