select u.id, string_agg(p.title, ', ') as titles, max(p.published_at) as latest from users u left join posts p on p.user_id = u.id group by u.id
