select u.id, (select coalesce(json_arrayagg(p.id), json_array()) from posts p where p.user_id = u.id having count(*) > 100) as posts from users u
