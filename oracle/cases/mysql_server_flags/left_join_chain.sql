select t.id, o.label, p.label as second from t left join other o on o.t_id = t.id left join other p on p.t_id = o.id
