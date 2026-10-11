select json_arrayagg(id = 1) over () as l from users
