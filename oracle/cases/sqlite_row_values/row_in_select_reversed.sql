select id from users where (:created_at, :id) in (select created_at, id from users)
