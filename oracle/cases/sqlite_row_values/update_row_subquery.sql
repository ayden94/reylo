update users set (name, created_at) = (select :name, :created_at) where id = :id returning id, name
