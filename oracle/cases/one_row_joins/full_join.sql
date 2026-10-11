select a.n, b.m from (select count(*) as n from bar) a full join (select count(*) as m from users) b on a.n = b.m + 1
