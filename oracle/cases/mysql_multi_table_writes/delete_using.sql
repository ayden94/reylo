delete from u, e using users u join emails e on e.user_id = u.id where u.id = :id
