select lower(t) as lo, upper(t) as up, to_char(n, '999') as num, length(t) as len, date_trunc('day', ts) as day from rt
