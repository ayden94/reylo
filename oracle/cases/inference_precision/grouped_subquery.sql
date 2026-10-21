select u.id, (select jsonb_agg(p.title) from posts p where p.user_id = u.id group by p.user_id) as titles from users u group by u.id
