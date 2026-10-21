select id from users where (:created_at, :id) <= (created_at, id)
