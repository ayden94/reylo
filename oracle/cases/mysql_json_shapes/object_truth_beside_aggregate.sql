select json_object('t', id = 1, 'm', max(big), 'c', max(id) = 1, 'l', true) as o from users group by id
