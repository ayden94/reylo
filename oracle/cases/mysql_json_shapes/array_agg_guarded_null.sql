select coalesce(json_arrayagg(id), null) as ids from users where false
