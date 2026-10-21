select id from users where (created_at, id) is not (:created_at, :id)
