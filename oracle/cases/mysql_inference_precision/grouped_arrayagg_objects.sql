select u.id, json_arrayagg(json_object('title', p.title, 'at', p.published_at)) as posts from users u left join posts p on p.user_id = u.id where p.id > 0 group by u.id
