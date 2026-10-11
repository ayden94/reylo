select count(*) from bar limit coalesce(:n, 0)
