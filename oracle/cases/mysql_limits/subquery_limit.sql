select (select name from bar order by id limit 1) as first, count(*) as n from bar
