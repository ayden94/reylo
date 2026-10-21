select coalesce(:n, whole) as n, ifnull(big, :m) as m from everything
