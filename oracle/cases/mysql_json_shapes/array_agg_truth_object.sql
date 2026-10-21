select json_arrayagg(json_object('t', u.id = 1)) as l from users u join posts p group by p.tag
