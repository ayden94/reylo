select c.body, p.title, u.name from comments c left join posts p on p.id = c.post_id left join users u on u.id = p.user_id
