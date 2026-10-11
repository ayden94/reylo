select (select count(name) from posts where false) as n from users
