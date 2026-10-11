select n, count(*) as c from t where n between -1 and 2 group by n having count(*) > 1 and max(maybe_n) is null
