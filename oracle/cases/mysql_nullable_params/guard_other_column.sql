with input as (select :name as name, 1 as one) select id from users, input where input.one is null or users.name = input.name
