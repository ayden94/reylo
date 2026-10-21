select json_object('id', u.id, 'n', (select count(p.id) from posts p where p.user_id = u.id)) as o from users u
