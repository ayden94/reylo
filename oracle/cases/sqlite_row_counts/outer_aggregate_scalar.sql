select (select count(u.id) from posts where 0) as n from users u
