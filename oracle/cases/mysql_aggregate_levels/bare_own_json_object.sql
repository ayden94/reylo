select (select coalesce(json_arrayagg(json_object('title', title)), json_array()) from posts where false) as titles from users
