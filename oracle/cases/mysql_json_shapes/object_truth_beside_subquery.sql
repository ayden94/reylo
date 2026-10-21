select json_object('t', u.id = 1, 'posts', (select coalesce(json_arrayagg(json_object('id', p.id, 'first', p.id = 1)), json_array()) from posts p where p.user_id = u.id)) as o from users u
