with recursive c(n) as (select 1 union all select n + 1 from c where n < :top) select count(*) as n from c
