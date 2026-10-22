select jsonb_agg(payload) as payloads, json_agg(id) as ids from events where id > 100
