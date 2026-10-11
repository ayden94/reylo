select j.a, json_object('a', j.a) as o from users u, json_table(u.doc, '$' columns (a int path '$.a')) j
