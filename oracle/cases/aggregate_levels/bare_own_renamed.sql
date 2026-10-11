select (select count(b) from posts p (a, b, c) where false) as n from users
