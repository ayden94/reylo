select id from users where id = :id intersect select user_id from posts limit 10
