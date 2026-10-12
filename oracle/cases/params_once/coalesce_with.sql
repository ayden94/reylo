with input as (select :age::int as age) select id from users, input where users.age = coalesce(input.age, users.age)
