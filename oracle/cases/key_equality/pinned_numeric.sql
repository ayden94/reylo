select distinct u.email from pv_posts p join pv_users u on u.id = length(p.amount::text) where p.amount = 1
