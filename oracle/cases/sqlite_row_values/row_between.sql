select id from users where (created_at, id) between (:from_at, :from_id) and (:to_at, :to_id)
