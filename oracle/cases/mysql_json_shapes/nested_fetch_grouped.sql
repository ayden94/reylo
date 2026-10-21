select u.id, (select coalesce(json_arrayagg(p.id), json_array()) from posts p where p.user_id = u.id group by p.user_id) as posts from users u
