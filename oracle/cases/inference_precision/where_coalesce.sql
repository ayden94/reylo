select u.name, p.title, p.published_at from users u left join posts p on p.user_id = u.id where coalesce(p.published_at, now()) > '2000-01-01'
