select (select coalesce(json_arrayagg(u.name), json_array()) from posts where false) as names from users u
