select u.email, count(*) as n from pv_posts p join pv_users u on u.id = ascii(p.tag) where p.tag = 'a' group by u.email
