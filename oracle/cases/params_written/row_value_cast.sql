update users set (name, email, n) = (:name::text, :email::text, :n::int) where id = :id::int returning name, email, n
