select json_arrayagg(json_object('dir', concat(name, '\'), 'id', id)) as l from users
