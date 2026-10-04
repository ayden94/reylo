select distinct u.email from wr_posts p join wr_users u on u.id = (p).author_id
