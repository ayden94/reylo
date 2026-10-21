select json_build_object('n', (select count(*) from posts q where q.user_id = x.id)) as j from users x
