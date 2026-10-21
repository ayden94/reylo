with p (i, t) as (select id, title from posts) select (select count(t) from p where false) as n from users
