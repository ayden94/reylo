select a.n, u.name from (select count(*) as n from users) a, users u
