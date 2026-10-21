select s.n from (select count(*) as n from bar) s group by rollup (s.n)
