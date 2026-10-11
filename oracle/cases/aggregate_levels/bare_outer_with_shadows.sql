with posts as (select id from comments) select (select count(title) from posts where false) as n from users
