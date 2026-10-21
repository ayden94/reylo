select json_object('o', json_object('t', id = 1), 'm', max(big)) as o from users group by id
