select d.total, d.top from (select sum(n) as total, max(n) as top from t where n between -1 and 2) d
