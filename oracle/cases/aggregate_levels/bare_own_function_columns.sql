select (select count(x) from generate_series(1, 0) g (x)) as n from users
