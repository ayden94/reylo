select coalesce(json_arrayagg(id), 0) as ids from users where false
