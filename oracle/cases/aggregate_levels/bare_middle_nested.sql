select (select (select count(body) from posts where false) from comments) as n from users
