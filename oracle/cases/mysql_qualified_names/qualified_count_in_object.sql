select json_object('n', (select reylo_oracle_app.count(p.id) from posts p where false)) as o from users u
