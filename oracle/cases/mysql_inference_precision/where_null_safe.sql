select u.name, p.title from users u left join posts p on p.user_id = u.id where not (p.title <=> 'x')
