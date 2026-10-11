select t.id, t.name, o.id as other_id, o.label, length(t.name) as len, t.n + 1 as next from t right join other o on o.t_id = t.id
