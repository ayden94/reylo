select (select s.n from (select count(u.id) as n from posts where 0) s) as n from users u
