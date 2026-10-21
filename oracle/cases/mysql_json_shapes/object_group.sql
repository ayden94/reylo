select json_object('id', u.id, 'n', count(p.id), 'last', max(p.id)) as o from users u left join posts p on p.user_id = u.id group by u.id
