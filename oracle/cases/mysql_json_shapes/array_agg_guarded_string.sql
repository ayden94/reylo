select coalesce(json_arrayagg(id), '[]') as ids from users where false
