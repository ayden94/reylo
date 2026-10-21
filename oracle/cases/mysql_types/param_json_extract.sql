select json_extract(:doc, '$.a') as a, json_unquote(json_extract(:doc2, '$.a')) as b, json_length(:doc3) as n
