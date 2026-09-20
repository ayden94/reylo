with recent as (
  select id, title, body, published_at from pgf_posts where published_at is not null
)
select id, title, body, published_at from recent
