select jsonb_agg(row(d.id, '[1]'::jsonb)::docs) as made from docs d
