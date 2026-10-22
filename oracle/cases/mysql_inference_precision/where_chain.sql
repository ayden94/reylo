select u.name, p.title, c.body from users u left join posts p on p.user_id = u.id left join comments c on c.post_id = p.id where c.id > 0
