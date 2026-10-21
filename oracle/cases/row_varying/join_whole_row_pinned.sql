select u.email, p.title from wr_posts p join wr_users u on u.id = (p).author_id where u.id = :id
