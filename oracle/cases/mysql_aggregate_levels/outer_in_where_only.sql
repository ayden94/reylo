select u.id, (select count(*) from posts p where p.title = u.name) as n from users u
