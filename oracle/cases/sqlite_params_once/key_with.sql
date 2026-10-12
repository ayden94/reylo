with input as (select cast(:id as integer) as id) select users.id, name from users, input where users.id = input.id
