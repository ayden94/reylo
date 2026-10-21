select case when x is not null then array_agg(x) over (order by id) end as arr from w
