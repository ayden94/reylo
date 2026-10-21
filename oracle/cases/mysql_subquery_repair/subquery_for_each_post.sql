select p.id, p.title, (select u.name from users u where u.id = p.user_id) as author, (select count(*) from posts q where q.user_id = p.user_id) as siblings from posts p
