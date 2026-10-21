select email from users where id = (select author_id from posts where id = :id)
