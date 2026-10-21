select u.id, (select reylo_oracle_app.count(p.id) from posts p where false) as n from users u
