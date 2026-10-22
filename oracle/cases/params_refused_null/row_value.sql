update accounts set (email, nickname) = (:email, :nickname) where id = :id returning id
