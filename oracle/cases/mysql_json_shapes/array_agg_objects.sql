select json_arrayagg(json_object('id', id, 'name', name)) as list from users
