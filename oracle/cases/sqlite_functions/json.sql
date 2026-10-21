select json_extract(doc, '$.a') as a, json_array_length(doc) as l, json_valid(doc) as v, json_object('n', name) as o, (select count(*) from json_each(doc)) as items from users where json_valid(doc)
