with recent as (
  select id, user_id, title from pgf_posts where published_at is not null
)
select u.id, u.name, r.title
from pgf_users u join recent r on r.user_id = u.id
where u.id = :id
