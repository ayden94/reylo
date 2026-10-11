select (select json_group_array(title) from posts where 0) as titles from users
