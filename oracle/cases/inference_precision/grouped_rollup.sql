select kind, jsonb_agg(id) as ids from events where id > 100 group by rollup (kind)
