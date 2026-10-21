select s.n, b.id from (select count(*) as n from tags) s right join bar b on false
