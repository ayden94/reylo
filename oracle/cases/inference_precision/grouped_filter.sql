select u.id, jsonb_agg(p.title) filter (where p.published_at is not null) as titles, array_agg(p.id) filter (where p.id > 1) as ids from users u left join posts p on p.user_id = u.id group by u.id
