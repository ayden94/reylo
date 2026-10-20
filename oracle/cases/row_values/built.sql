select jsonb_agg(row(null, null, 'x')::docs) as built from docs
