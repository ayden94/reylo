select p.title, u.email from posts p left join users u on u.id = p.author_id where p.id = :id
