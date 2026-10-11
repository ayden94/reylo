select json_build_object('p', (select $1::text from posts q where false)) as j from users x
