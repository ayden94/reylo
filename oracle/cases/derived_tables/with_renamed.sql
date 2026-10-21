with c as (select max(name) as b, count(*) as a from bar where false) select x.a, x.b from c as x (a, b)
