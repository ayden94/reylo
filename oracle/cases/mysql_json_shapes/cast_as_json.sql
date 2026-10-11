select cast(json_object('id', id) as json) as o, cast(name as json) as n from users where name = '1'
