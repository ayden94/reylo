select json_build_object('n', (select count(title) from posts where user_id = x.id)) as j from users x
