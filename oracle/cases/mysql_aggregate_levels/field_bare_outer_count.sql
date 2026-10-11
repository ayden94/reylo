select json_object('n', (select count(name) from posts where false)) as o from users
