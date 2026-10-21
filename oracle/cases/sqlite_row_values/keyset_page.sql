select id, name, created_at from users where (created_at, id) < (:created_at, :id) order by created_at desc, id desc limit 20
