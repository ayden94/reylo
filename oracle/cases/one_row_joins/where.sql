select a.n, b.m from (select count(*) as n from bar) a, (select count(*) as m from users) b where a.n = b.m
