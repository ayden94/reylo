update counters set n = (with recursive c(n) as (select 1 union all select n + 1 from c) select count(*) from c)
