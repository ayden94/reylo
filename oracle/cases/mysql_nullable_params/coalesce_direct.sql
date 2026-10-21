select id, name from users where coalesce(:email, email) = email or ifnull(:bio, 'x') = bio
