with input as (select cast(:min as integer) as min) select id from users, input where input.min is null or age > input.min
