with input as (select cast(:n as signed) as n) select whole + input.n as bucket, count(*) as c from everything, input where whole between 0 and 2 group by whole + input.n
