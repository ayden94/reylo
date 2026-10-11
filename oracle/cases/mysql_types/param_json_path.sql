select id, json_extract(doc, :path) as found from everything where json_extract(doc, :path) is not null
