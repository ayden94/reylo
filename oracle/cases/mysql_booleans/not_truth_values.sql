select 1 as one, 5 as five, case when done then 1 else 0 end as cs, if(done, 1, a = b) as mixed, (a = b) + 0 as plus from flags
