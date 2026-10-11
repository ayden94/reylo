select count(*) as n, count(email) as e, group_concat(name, ',') as names, total(score) as t, max(score) as m, min(name) as first, avg(score) as mean from users
