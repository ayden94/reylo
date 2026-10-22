update accounts set email = :email, nickname = cast(:nickname as char) where id = :id
