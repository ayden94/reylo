select coalesce(json_arrayagg(id), json_array(null)) as ids from users where false
