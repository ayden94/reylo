select id, now() + interval :n day as later from users where id = :id
