select u.id, (select count(*) from posts p where p.user_id = u.id union select 1 limit 1) as n from users u
