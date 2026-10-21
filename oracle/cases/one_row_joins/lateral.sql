select a.n, b.m from (select count(*) as n from bar) a cross join lateral (select count(*) as m from users where id > a.n) b
