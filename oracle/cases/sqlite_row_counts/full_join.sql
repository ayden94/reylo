select a.n, b.m from (select count(*) as n from users) a full join (select count(*) as m from posts) b on a.n = b.m + 1
