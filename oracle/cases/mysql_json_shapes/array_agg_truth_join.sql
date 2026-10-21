select json_arrayagg(u.id = 1) as l from users u join posts p group by p.tag
