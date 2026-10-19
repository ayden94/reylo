select coalesce(jsonb_agg(e.payload), '[]'::jsonb) as payloads from events e
