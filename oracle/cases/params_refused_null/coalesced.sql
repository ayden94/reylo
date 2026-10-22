update accounts set nickname = coalesce(:nickname::text, nickname), email = coalesce(:email, email) where id = :id returning nickname, email
