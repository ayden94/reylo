select (select s.n from (select count(name) as n from posts where 0) s) as n from users
