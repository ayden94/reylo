select coalesce(json_arrayagg(id), cast('[]' as json)) as ids from users where false
