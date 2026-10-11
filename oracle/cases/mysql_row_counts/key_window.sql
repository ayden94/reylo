select id, row_number() over () as r from users where id = :id
