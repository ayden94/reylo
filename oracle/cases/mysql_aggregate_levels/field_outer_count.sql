select json_object('id', u.id, 'n', (select count(u.name) from posts where false)) as o from users u group by u.id
