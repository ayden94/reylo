select (select count(name) from generate_series(1, 0) g) as n from users
