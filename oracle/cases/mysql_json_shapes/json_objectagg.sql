select json_objectagg(name, id) as by_name from users where name <> ''
