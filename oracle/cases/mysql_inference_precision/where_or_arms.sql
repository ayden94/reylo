select u.name, p.title from users u left join posts p on p.user_id = u.id where p.id = 1 or p.id in (2, 3)
