select p.title, u.name from posts p left join users u on u.id = p.user_id where p.id = :id
