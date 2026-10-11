select t.o from (select json_object('id', id) as o from users) t
