select u.id, u.name,
  coalesce((select jsonb_agg(jsonb_build_object('id', p.id, 'title', p.title, 'body', p.body, 'at', p.published_at)) from lando_posts p where p.user_id = u.id), '[]') as posts,
  to_jsonb(u) as row,
  (select count(*) from lando_posts p where p.user_id = u.id) as post_count
from lando_users u
where u.id = :id
