select (select count(t) from (select id, title from posts) p (i, t) where false) as n from users
