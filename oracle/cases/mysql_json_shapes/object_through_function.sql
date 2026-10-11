select json_merge_preserve(json_object('id', id), json_object('id', email)) as merged, json_insert(json_object('id', id), '$.x', 1) as inserted from users
