select u.id, (select count(*) from posts p where p.user_id = u.id group by p.user_id) as n from users u
