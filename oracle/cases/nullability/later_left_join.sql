select p.title, u.name from posts p left join users u on u.id = p.author_id left join teams t on t.id = u.team_id
