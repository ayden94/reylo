update accounts set nickname = coalesce(cast(:nickname as text), nickname), email = ifnull(:email, email) where id = :id returning nickname, email
