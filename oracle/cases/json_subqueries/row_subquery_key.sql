select id, (select row_to_json(q) from posts q where q.id = users.id) as j from users
