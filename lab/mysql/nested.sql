select u.id,
  (select coalesce(json_arrayagg(json_object('id', p.id, 'title', p.title, 'tag', p.tag)), json_array())
   from posts p where p.user_id = u.id) as posts,
  json_object('email', u.email, 'flag', u.flag) as meta,
  (select count(*) from posts p where p.user_id = u.id) as n_posts,
  (select max(p.id) from posts p where p.user_id = u.id) as last_post
from users u
where u.id = :id
