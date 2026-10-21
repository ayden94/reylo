select coalesce((select json_object('id', p.id) from posts p where p.user_id = u.id limit 1), json_object('id', 0)) as first from users u
