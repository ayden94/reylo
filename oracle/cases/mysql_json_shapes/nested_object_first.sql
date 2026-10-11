select u.id, (select json_object('id', p.id, 'title', p.title) from posts p where p.user_id = u.id order by p.id limit 1) as first from users u
