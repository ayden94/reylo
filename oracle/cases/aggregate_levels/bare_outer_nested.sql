select (select count(c.id) + (select count(name) from posts where false) from comments c) as n from users
