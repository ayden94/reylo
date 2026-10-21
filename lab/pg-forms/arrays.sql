select u.id,
  array(select p.id from pgf_posts p where p.user_id = u.id) as post_ids,
  array(select p.title from pgf_posts p where p.user_id = u.id) as post_titles,
  (select array_agg(p.id) from pgf_posts p where p.user_id = u.id) as agg_ids,
  (select array_agg(row(p.id, p.title)) from pgf_posts p where p.user_id = u.id) as agg_rows
from pgf_users u
where u.id = :id
