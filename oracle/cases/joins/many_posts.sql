select p.title from posts p join users u on u.id = p.author_id where u.id = :id
