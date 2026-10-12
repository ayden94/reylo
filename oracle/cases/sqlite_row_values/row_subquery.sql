select id, name from users where (created_at, id) = (select :created_at, :id)
