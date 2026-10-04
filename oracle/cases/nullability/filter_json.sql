select u.id, json_agg(json_build_object('id', p.id, 'title', p.title)) filter (where p.id is not null) as posts from users u left join posts p on p.author_id = u.id group by u.id
