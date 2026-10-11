select json_object('user', u.id, 'post', p.id) as o from users u right join posts p on p.user_id = u.id
