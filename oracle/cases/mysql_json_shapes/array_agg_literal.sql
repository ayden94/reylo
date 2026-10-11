select json_arrayagg(true) as l from users u join posts p group by p.tag
