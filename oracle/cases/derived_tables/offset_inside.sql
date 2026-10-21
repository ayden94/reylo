select s.n from (select count(*) as n from bar offset 1) s
