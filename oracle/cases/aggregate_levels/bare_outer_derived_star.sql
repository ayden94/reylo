select (select count(name) from (select * from posts) p where false) as n from users
