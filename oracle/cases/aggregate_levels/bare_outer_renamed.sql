select (select count(name) from posts p (a, b, c) where false) as n from users
