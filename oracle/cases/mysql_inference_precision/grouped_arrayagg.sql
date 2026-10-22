select kind, json_arrayagg(payload) as payloads from events group by kind
