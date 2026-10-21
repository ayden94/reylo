select u.id, count(*) as n from wr_posts p join wr_users u on u.id = (p).author_id group by u.id
