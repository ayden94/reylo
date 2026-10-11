select id from users where id = :id and id in (select user_id from posts)
