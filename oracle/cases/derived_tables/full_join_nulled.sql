select s.n, t.m from (select count(*) as n from bar) s full join (select count(*) as m from tags) t on false
