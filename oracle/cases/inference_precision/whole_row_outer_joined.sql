select u.id, jsonb_agg(p) as posts, array_agg(p) as rows from users u left join posts p on p.user_id = u.id group by u.id
