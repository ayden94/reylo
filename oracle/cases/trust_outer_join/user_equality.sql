select p.title, u.name, u.email from posts p left join users u on u.id = p.editor_id where u.name = 3
