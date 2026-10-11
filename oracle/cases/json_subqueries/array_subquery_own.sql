select json_build_object('titles', (select coalesce(json_agg(title), '[]') from posts where user_id = x.id)) as j from users x
