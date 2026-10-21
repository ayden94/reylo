select id, (select to_jsonb(p) from posts p where p.user_id = users.id limit 1) as j from users
