select 1 as one, s.*, 2 as two from (select count(*) as n, max(name) as top from bar where false) s
