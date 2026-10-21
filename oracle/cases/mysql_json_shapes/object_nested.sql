select json_object('user', json_object('id', id, 'name', name), 'meta', json_object('email', email)) as o from users
