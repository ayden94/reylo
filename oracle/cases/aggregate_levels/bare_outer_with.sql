with p as (select id, title from posts) select (select count(name) from p where false) as n from users
