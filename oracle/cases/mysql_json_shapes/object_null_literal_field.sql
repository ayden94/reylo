select json_object('a', null, 'b', cast(null as signed), 'c', nullif(id, id)) as o from users
