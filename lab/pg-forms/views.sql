select m.id, m.name, m.email, s.posts, s.last_at
from pgf_members m
join pgf_user_stats s on s.id = m.id
