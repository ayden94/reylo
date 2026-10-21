select (select coalesce(json_arrayagg(title), json_array()) from posts where false) as titles from users
