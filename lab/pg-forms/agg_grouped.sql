select p.user_id,
  count(*) as n,
  sum(p.views) as total,
  count(*) filter (where p.views > 0) as viewed,
  count(*) over () as groups
from pgf_posts p
group by p.user_id
