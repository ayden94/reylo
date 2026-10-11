select (select json_group_array(json_object('name', name)) from posts where 0) as names from users
