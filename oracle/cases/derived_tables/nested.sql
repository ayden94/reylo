select s.n from (select t.n from (select count(*) as n from bar) t) s
