with p as (select id, title from posts) select (select count(name) from p where 0) as n from users
