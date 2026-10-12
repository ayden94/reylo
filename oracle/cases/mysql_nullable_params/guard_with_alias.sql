with input as (select cast(:min as signed) as min) select id from users, input i where i.min is null or users.id >= i.min
