update users u, emails e set u.name = :name, e.email = :email where e.user_id = u.id and u.id = :id
