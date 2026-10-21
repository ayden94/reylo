select (select count(title) from posts where 0 limit 5) as n from users
