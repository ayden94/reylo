select a.n, b.m from (select count(*) as n from users) a, (select count(*) as m from posts) b
