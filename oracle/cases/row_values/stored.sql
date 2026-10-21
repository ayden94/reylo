select jsonb_agg(d order by d.id) as stored from docs d
