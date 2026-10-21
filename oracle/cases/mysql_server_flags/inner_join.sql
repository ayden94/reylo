select t.id, t.name, o.id as other_id, o.label, o.maybe_label from t join other o on o.t_id = t.id
