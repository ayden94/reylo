select done + 0 as plus, coalesce(starred, 0) as zero, max(done) as most, case when done then 1 else 0 end as cs from todos
