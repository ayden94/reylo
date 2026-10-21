select u.id, (select s.n from (select count(p.id) as n from posts p where 0) s) as n from users u
