select json_arrayagg(payload) as payloads from events where id > 100
