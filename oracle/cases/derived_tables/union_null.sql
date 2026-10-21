select s.n from (select count(*) as n from bar) s union all select null
