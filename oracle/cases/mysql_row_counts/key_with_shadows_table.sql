with users as (select 1 as id union all select 1) select id from users where id = :id
