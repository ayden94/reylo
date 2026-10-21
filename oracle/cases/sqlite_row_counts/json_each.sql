select a.n, j.value from (select count(*) as n from users) a, json_each('[1, 2]') j
