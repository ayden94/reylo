select u.id, p.id as post, (select count(*) from posts q where q.user_id = p.user_id) as n from users u right join posts p on p.user_id = u.id
