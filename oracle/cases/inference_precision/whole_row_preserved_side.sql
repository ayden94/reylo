select jsonb_agg(u) as users from users u left join posts p on p.user_id = u.id
