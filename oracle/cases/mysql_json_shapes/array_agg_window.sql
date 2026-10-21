select json_arrayagg(id) over () as ids from users
