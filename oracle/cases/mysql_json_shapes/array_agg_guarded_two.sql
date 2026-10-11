select coalesce(json_arrayagg(id), json_array(), cast(' [ ] ' as json)) as ids from users where false
