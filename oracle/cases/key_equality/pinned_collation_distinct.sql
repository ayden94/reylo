select distinct u.email from pv_posts p join pv_users u on u.id = ascii(p.tag) where p.tag = 'a'
