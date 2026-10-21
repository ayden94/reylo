select json_object('id', id) as o from users union all select json_object('id', email) from users
