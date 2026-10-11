with a as (select count(*) as n from bar) select x.n from a x join a y on x.n = y.n + :d
