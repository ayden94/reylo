select case when x is not null then coalesce(array_agg(x) over (order by id), '{}') else '{}' end as arr from w
