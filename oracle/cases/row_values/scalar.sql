select json_build_object('first', (select c from docs c order by c.id limit 1)) as o
