select (select json_group_array(u.name) from posts where 0) as names from users u
