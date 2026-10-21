select json_build_object('first', (select q.title from posts q where q.id = x.id)) as j from users x
