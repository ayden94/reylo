select (select count(c.id) + (select count(name) from posts where 0) from comments c) as n from users
