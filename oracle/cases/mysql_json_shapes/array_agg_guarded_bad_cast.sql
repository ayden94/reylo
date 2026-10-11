select coalesce(json_arrayagg(id), cast('x' as date)) as ids from users where false
