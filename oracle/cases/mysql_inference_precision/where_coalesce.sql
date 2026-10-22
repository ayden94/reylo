select u.name, p.title, p.published_at from users u left join posts p on p.user_id = u.id where coalesce(p.published_at, '2000-01-01') > '1999-01-01'
