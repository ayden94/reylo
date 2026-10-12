with input as (select cast(:min as integer) as min) select id from users, input where min is null or age > min
