select json_object('e', u.id = 1, 'n', (select count(u.id) from posts p where p.id = 1)) as o from users u group by u.id
