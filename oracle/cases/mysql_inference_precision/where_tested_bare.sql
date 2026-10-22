select p.title, published_at, p.published_at as at from users u left join posts p on p.user_id = u.id where published_at is not null
