select (select coalesce(json_agg(u.name), '[]') from posts where false) as names from users u
