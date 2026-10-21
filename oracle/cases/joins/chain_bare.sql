select body, title, email from comments c join posts p on p.id = post_id join users u on u.id = author_id where c.id = :id
