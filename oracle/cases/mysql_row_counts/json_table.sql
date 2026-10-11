select j.a from json_table('[{"a": 1}, {"a": 2}]', '$[*]' columns (a int path '$.a')) j
