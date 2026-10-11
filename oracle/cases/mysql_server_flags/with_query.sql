with w as (select name, maybe_name, n + 1 as next from t where n between -1 and 2) select name, maybe_name, next from w
