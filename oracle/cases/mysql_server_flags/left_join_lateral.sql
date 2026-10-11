select t.id, l.c, l.m from t left join lateral (select count(*) as c, max(label) as m from other o where o.t_id = t.id) l on true
