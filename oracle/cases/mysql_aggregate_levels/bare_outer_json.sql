select (select coalesce(json_arrayagg(name), json_array()) from posts where false) as names from users
