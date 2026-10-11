select (select count(distinct name) from posts where false) as n from users
