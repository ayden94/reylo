update accounts set email = :email, nickname = cast(:nickname as text) where id = :id returning email, nickname
