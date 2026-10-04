select row_number() over () as r, rank() over (order by id) as k, lag(name, 1, 'none') over () as prev, lag(name) over () as prev_null from users
