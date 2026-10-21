with a as (select count(*) as n from bar), b as (select count(*) as n from users) select n from a join b using (n)
