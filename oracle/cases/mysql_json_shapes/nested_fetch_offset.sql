select u.id, (select coalesce(json_arrayagg(p.id), json_array()) from posts p where p.user_id = u.id limit 1 offset 1) as posts from users u
