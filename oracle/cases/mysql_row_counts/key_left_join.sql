select u.id, p.title from users u left join posts p on p.user_id = u.id where u.id = :id
