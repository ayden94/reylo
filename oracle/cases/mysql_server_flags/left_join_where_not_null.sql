select t.id, o.id as other_id, o.label from t left join other o on o.t_id = t.id where o.id is not null
