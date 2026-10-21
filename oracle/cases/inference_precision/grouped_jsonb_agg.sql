select kind, jsonb_agg(payload order by id) as payloads from events group by kind
