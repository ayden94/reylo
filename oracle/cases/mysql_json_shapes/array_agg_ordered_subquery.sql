select (select coalesce(json_arrayagg(t.title), json_array()) from (select title from posts where user_id = u.id order by id) t) as titles from users u
