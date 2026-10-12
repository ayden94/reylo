with input as (select :min::int as min) select u.id from users u, input i where i.min is null or u.age > i.min
