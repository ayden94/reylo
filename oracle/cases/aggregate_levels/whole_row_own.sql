select (select count(posts) from posts where false) as n from users
