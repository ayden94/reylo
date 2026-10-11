select json_arrayagg(cast(u.id = 1 as json)) as l from users u join posts p group by p.tag
