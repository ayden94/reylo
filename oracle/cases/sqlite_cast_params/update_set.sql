update users set email = CAST(:email AS TEXT), name = CAST(:name AS TEXT), n = CAST(:n AS INTEGER), doc = json(:doc) where id = CAST(:id AS INTEGER) returning email, n, doc
