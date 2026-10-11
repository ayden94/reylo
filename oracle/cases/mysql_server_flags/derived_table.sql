select d.name, d.maybe_name, d.up, d.c from (select name, maybe_name, upper(name) as up, coalesce(maybe_name, 'x') as c from t) d
