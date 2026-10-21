update users set n = coalesce(CAST(:n AS INTEGER), n), doc = coalesce(json(:doc), doc) where json(:guard) is null or id = CAST(:id AS INTEGER) returning n, doc
