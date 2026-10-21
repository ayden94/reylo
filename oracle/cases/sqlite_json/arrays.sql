select json_array(id, name, email, score, json_array(), json_object()) as a,
  json_object(name, id) as keyed,
  json_object('k', id, 'k', name) as twice,
  (select json_group_array(p.body) from posts p where p.user_id = users.id) as bodies,
  (select json_group_object(p.title, p.id) from posts p where p.user_id = users.id) as by_title,
  (select json_group_array(json_array(p.id, p.title)) from posts p where p.user_id = users.id) as pairs
from users
