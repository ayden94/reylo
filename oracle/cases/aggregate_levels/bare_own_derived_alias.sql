select (select count(t) from (select id, title as t from posts) p where false) as n from users
