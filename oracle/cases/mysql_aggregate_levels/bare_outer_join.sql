select (select count(name) from posts p join comments c on c.post_id = p.id where false) as n from users
