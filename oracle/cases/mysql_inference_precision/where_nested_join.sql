select u.name, p.title, c.body from users u left join (posts p join comments c on c.post_id = p.id) on p.user_id = u.id where c.body like 'F%'
