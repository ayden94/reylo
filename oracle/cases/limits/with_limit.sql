with c as (select count(*) as n from bar limit 3) select n from c
