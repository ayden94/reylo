select (select count(u.id) from posts where 0 limit 5) as n from users u
