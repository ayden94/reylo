select u.id, upper(u.name) as shout, u.score + 1 as next, (select count(*) from posts p where p.user_id = u.id) as n from users u
