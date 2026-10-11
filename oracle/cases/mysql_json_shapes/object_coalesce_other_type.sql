select json_object('n', coalesce(big, 'none'), 'm', ifnull(email, 0)) as o from users
