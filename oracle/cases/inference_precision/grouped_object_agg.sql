select kind, jsonb_object_agg(id, payload) as by_id, json_object_agg(id, note) as notes from events group by kind
