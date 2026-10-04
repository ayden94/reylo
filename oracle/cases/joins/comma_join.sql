select p.title, u.email from posts p, users u where u.id = p.author_id and p.id = :id
