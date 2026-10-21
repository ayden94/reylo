select (select count(title) from (select * from posts) p where false) as n from users
