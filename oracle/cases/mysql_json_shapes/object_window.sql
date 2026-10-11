select json_object('id', id, 'rank', rank() over (order by id), 'previous', lag(id) over (order by id)) as o from users
