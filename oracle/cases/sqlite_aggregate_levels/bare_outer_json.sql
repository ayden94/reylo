select (select json_group_array(name) from posts where 0) as names from users
