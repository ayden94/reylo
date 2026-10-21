select users.id, to_jsonb(q) as j from users left join posts q on q.user_id = users.id
