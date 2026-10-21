select j.a from users u, json_table(u.doc, '$' columns (a int path '$.a', nested path '$.b[*]' columns (b int path '$'))) j
