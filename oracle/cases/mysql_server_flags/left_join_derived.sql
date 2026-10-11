select t.id, d.c, d.m from t left join (select t_id, count(*) as c, max(label) as m from other group by t_id) d on d.t_id = t.id
