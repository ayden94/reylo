select u.id, l.n from users u, lateral (select count(*) as n from posts p where p.user_id = u.id) l where u.id = :id
