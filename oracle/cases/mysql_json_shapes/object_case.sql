select case when flag then json_object('id', id) else json_object('name', name) end as o from users
