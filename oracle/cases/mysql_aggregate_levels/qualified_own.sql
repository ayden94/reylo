select (select count(p.title) from posts p where false) as n from users u
