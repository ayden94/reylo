select t.id, (select count(*) from other o where o.t_id = t.id) as n, t.name, upper(t.name) as shout, t.maybe_name from t
