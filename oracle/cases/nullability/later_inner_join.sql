select p.title, u.name, t.name as team from posts p left join users u on u.id = p.author_id join teams t on t.id = u.team_id
