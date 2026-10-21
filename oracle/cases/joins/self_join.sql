select a.email, b.name from users a join users b on b.id = a.id where a.id = :id
