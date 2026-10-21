select coalesce(n > 1, done) as co, iif(starred, n = 1, null) as iff, case when n > 0 then done end as cs, (select t.n > 1 from todos t order by t.id limit 1) as sub from todos
