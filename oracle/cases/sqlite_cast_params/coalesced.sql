update users set n = coalesce(CAST(:n AS INTEGER), n), doc = coalesce(json(:doc), doc) where coalesce(json(:guard), 'none') = 'none' or id = CAST(:id AS INTEGER) returning n, doc
