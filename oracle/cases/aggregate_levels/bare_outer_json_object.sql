select (select coalesce(json_agg(json_build_object('name', name)), '[]') from posts where false) as names from users
