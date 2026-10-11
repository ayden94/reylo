select coalesce(json_arrayagg(id), null, json_array()) as ids from users where false
