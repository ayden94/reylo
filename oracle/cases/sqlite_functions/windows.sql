select name, row_number() over (order by id) as r, lag(name) over (order by id) as prev, sum(score) over (partition by name) as s, count(*) filter (where score > 0) over () as positive from users
