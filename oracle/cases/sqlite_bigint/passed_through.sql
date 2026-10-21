select x.user_id, v.user_id as through_view, max(a.user_id) as most, min(a.ref) as least, coalesce(a.ref, 0) as co, ifnull(a.ref, a.n) as ifn,
  case when a.n > 0 then a.user_id else 0 end as cs, +a.user_id as pos, nullif(a.big, 0) as nul, (select max(user_id) from accounts) as sub
from accounts a, (select user_id from accounts) x, account_users v
group by a.id
