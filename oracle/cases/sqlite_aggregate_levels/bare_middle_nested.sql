select (select (select count(body) from posts where 0) from comments) as n from users
