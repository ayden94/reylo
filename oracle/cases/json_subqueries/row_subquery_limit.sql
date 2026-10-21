select id, (select to_jsonb(q) from posts q where q.user_id = users.id limit 1) as j from users
