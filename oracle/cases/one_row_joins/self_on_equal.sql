with a as (select count(*) as n from bar) select x.n, y.n as m from a x join a y on 1 = 1
