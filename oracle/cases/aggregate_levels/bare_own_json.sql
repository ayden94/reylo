select (select coalesce(json_agg(title), '[]') from posts where false) as titles from users
