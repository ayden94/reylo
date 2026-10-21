select json_build_object('first', (select q.title from posts q where q.user_id = x.id limit 1)) as j from users x
