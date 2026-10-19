select jsonb_agg(d order by d.id) as all_docs from docs d
