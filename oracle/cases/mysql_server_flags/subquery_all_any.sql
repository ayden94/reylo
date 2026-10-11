select id from t where n > all (select n from nothing) and n = any (select t_id from other)
