select u.id, (select count(id) from posts where user_id = u.id) as n from users u
