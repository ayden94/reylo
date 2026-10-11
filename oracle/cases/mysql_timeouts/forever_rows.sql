with recursive n (i) as (select 1 union all select i + 1 from n where i < 100000000) select /*+ set_var(cte_max_recursion_depth = 4294967295) */ count(*) as c from n
