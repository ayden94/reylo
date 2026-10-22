update users set (name, email) = (:name, :email) where id = :id returning name, email
