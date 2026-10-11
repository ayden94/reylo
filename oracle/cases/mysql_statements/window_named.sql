select id, rank() over w as r from users window w as (order by id)
