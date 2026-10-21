select u.id, p.id as post, p.title, (select count(*) from posts q where q.user_id = u.id) as n from users u left join posts p on p.user_id = u.id
