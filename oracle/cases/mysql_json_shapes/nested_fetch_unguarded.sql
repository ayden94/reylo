select u.id, (select json_arrayagg(json_object('id', p.id)) from posts p where p.user_id = u.id) as posts from users u
