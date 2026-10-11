select coalesce(json_arrayagg(id), json_object()) as ids from users where false
