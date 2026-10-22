select u.name, p.title from posts p right join users u on p.user_id = u.id where lower(p.title) like 'h%'
