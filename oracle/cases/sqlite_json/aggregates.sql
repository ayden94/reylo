select u.id,
  json_group_array(json_object('id', p.id, 'title', p.title)) filter (where p.id is not null) as posts,
  json_group_array(p.title) as titles,
  json_group_object(coalesce(p.title, ''), p.id) as ids
from users u left join posts p on p.user_id = u.id
group by u.id
