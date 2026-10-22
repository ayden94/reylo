select id, json_arrayagg(id) over (order by id rows between 1 following and 1 following) as next from events group by id
