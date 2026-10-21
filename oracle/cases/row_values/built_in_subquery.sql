select json_build_object('x', (select row(null, null, 'n')::docs from docs limit 1)) as o
