with p as (select id, title from posts) select (select count(title) from p where false) as n from users
