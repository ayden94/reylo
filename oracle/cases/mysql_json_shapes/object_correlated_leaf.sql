select u.id, (select json_object('post', p.id, 'author', u.name, 'email', u.email) from posts p where p.user_id = u.id limit 1) as o from users u
