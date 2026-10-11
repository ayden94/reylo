select (select group_concat(name order by title) from posts where false) as n from users
