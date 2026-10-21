select (select count(title) from (select id, title from posts) p where 0) as n from users
