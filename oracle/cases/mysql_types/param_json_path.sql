select id, found from (select id, json_extract(doc, :path) as found from everything) as e where found is not null
