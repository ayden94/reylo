with input as (select cast(:n as signed) as n) select id + input.n as bucket, count(*) as c from users, input where id between 0 and 2 group by id + input.n
