select json_object('any', id = any (select user_id from posts), 'all', id > all (select user_id from posts)) as o from users
