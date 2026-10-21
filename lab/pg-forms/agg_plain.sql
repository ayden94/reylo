select count(*) as n,
  sum(p.views) as total,
  avg(p.views) as mean,
  max(p.published_at) as latest,
  string_agg(p.title, ',') as titles,
  bool_and(p.views > 0) as all_viewed,
  count(*) filter (where p.published_at is not null) as published
from pgf_posts p
