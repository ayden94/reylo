update users set (name, email) = (CAST(:name AS TEXT), CAST(:email AS TEXT)) where id = CAST(:id AS INTEGER) returning name, email
