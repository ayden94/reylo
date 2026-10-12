update users u join emails e on e.user_id = u.id set e.email = :email where u.id = :id
