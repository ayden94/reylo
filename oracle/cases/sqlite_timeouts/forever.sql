with recursive c(n) as (select 1 union all select n + 1 from c) select count(*) as n from c
