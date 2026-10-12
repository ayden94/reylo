select id from users where (created_at, id) in ((:at1, :id1), (:at2, :id2))
