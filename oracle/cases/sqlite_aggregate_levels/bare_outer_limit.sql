select (select count(name) from posts where 0 limit 5) as n from users
