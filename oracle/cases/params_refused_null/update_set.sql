update accounts set email = :email, nickname = :nickname, tag = :tag, note = :note::text where id = :id returning email, nickname
