select u.id, jsonb_agg(p) as posts from users u left join posts p on p.user_id = u.id where p.id is not null group by u.id
