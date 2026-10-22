select jsonb_agg(e order by e.id) as events from events e
