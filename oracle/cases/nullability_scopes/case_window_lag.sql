select case when x is not null then lag(x, 1, 0) over (order by id) else 0 end as l from w
