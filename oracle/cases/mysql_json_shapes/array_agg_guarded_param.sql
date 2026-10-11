select coalesce(json_arrayagg(id), :fallback) as ids from users where false
