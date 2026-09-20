with ins as (
  insert into pgf_posts (user_id, title) values (:uid, :title) returning id, title, views
)
select id, title, views from ins
