select json_arrayagg(json_object('n', p.user_id, 'tag', p.tag)) as tags from posts p
