select t.id, o.label, p.label as second from t left join other o on o.t_id = t.id join other p on p.id = o.id
