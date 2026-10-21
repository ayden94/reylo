select u.id, (select count(*) from posts) as n from users u
