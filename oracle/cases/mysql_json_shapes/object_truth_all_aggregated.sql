select json_object('c', max(id) = 1, 'n', count(*), 'l', true, 't', 1 = 1, 'both', max(id) = 1 and min(id) = 1) as o from users
