select e.kind, jsonb_agg(e order by e.id) as events, array_agg(e) as rows from events e group by e.kind
