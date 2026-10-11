select (with users as (select 1 as id union all select 1) select count(*) from users where id = 1) as n
