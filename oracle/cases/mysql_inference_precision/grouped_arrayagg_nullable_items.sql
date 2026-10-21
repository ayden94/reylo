select u.id, json_arrayagg(p.published_at) as dates from users u left join posts p on p.user_id = u.id group by u.id
