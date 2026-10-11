select (select count(name) from (select id, title from posts) p where false) as n from users
