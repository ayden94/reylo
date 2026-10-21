select u.id, (select coalesce(json_arrayagg(json_object('id', p.id, 'title', p.title)), json_array()) from empty_posts p where p.user_id = u.id) as posts from users u
