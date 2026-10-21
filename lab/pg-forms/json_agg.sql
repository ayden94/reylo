select u.id,
  (select json_agg(json_build_object('id', p.id, 'title', p.title, 'body', p.body)) from pgf_posts p where p.user_id = u.id) as posts_json,
  (select jsonb_agg(jsonb_build_object('id', p.id, 'views', p.views)) from pgf_posts p where p.user_id = u.id) as posts_jsonb,
  coalesce((select jsonb_agg(jsonb_build_object('id', p.id, 'at', p.published_at)) from pgf_posts p where p.user_id = u.id), '[]') as posts_coalesced
from pgf_users u
where u.id = :id
