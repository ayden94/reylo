select json_object('t', u.id = 1, 'f', u.flag, 'titles', json_arrayagg(p.title)) as o from users u join posts p on p.user_id = u.id group by u.id
