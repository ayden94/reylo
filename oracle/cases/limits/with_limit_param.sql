with c as (select count(*) as n from bar limit :n) select n from c
