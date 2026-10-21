select json_object('any_value', any_value(id > 0)) as o from users group by id
