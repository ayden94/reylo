select `coalesce`(json_arrayagg(id), json_array()) as ids from users where false
