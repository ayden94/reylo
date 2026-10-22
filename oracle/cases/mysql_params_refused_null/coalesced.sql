update accounts set nickname = coalesce(cast(:nickname as char), nickname), email = ifnull(:email, email) where id = :id
