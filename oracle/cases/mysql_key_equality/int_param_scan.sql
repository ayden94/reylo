select id, name from users ignore index (primary) where id = :id
