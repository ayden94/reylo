select u.id, (select count(*) from posts p where p.user_id = u.id having count(*) > 100) as n from users u
