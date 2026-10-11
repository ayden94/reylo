with recursive n (i) as (select 1 union all select i + 1 from n where i < 5) select i from n
