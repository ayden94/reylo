select b.id, s.n, t.m from bar b left join ((select count(*) as n from bar) s cross join (select count(*) as m from tags) t) on false where b.id = :id
