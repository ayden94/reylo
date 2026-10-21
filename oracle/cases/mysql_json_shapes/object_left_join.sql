select json_object('user', u.id, 'post', p.id, 'title', p.title) as o from users u left join posts p on p.user_id = u.id
