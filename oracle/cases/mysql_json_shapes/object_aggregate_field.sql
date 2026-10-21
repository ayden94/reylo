select json_object('n', count(*), 'max', max(id), 'names', json_arrayagg(name)) as o from users
