select (select coalesce(json_agg(name), '[]') from posts where false) as names from users
