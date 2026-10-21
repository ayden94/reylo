select json_object('t', id = 1, 'w', row_number() over ()) as o from users
