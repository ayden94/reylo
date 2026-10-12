with input as (select :id::int as id) select users.id, name from users, input where users.id = input.id
