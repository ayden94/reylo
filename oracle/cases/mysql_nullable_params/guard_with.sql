with input as (select :name as name) select users.id, input.name from users, input where input.name is null or users.name = input.name
