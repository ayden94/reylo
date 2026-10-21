with p as (select id, title from posts) select (select count(title) from p where 0) as n from users
