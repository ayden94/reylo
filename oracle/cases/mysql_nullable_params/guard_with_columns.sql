with input (q) as (select :q) select id from users, input where q is not null and users.email like input.q
