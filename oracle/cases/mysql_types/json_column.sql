select doc, doc->'$.a' as a, doc->>'$.a' as a_text, json_extract(doc, '$.a') as e, json_type(doc) as kind from everything
