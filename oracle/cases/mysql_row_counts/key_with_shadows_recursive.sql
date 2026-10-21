with recursive users (id) as (select 1 union all select 1 union all select id + 1 from users where id < 2) select id from users where id = :id
