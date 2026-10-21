select id from users where id = coalesce(:a, :b)
