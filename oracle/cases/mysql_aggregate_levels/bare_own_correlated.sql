select u.id, (select count(title) from posts where user_id = u.id) as n from users u
