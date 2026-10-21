select json_arrayagg(coalesce(:p, score)) as l from users
