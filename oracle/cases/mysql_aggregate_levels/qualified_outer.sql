select (select count(u.name) from posts where false) as n from users u
