select u.name, p.title, p.published_at from users u left join posts p on p.user_id = u.id where p.published_at is not null or u.email is not null
