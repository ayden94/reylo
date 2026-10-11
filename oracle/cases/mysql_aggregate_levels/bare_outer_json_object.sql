select (select coalesce(json_arrayagg(json_object('name', name)), json_array()) from posts where false) as names from users
