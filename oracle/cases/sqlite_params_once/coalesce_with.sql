with input as (select cast(:age as integer) as age) select id from users, input where users.age = coalesce(input.age, users.age)
