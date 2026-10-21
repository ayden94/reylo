select (select count(title) from (select title from posts) p where false) as n from users
