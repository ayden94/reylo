select json_build_object('x', (select c from docs c, shadows limit 1)) as o
