select u.id, u.id in (select user_id from posts) as listed from users u
