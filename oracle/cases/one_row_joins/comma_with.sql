with a as (select count(*) as n from bar), b as (select count(*) as m from users) select a.n, b.m from a, b
