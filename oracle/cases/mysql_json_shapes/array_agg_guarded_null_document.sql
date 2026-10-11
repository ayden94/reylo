select coalesce(json_arrayagg(id), cast('null' as json)) as ids from users where false
