select (select count(name) from (select title from posts) p where false) as n from users
