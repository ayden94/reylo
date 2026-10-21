select json_arrayagg(doc) as docs, json_arrayagg(json_object('doc', doc)) as wrapped from users
