select json_object('t', id = 1, 'w', row_number() over (), 'c', row_number() over () = 1) as o from users
