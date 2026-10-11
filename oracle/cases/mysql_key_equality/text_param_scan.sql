select id from users ignore index (email) where email = :email
