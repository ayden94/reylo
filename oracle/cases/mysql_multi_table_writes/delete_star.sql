delete u.*, e.* from users u left join emails e on e.user_id = u.id where u.id = :id
