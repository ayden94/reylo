select (select count(name) + count(title) from posts where false) as n from users
