select p.title from posts p join users u on u.id = p.user_id where u.id = :id
