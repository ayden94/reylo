select json_build_object('n', (select count(name) from posts q where false)) as j from users x
