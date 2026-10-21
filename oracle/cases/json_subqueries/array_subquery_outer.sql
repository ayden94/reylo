select json_build_object('names', (select coalesce(json_agg(name), '[]') from posts where false)) as j from users x
