select :n = a as eq, :m in (a, b) as listed, :s like label as lk, coalesce(:p, a) = b as co from flags
