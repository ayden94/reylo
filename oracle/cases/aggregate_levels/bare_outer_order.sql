select (select count(1 order by name) from posts where false) as n from users
