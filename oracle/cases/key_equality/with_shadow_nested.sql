select t.id from (with users as (select 1::int8 as id union all select 1) select * from (select id from users where id = 1) s) t, users u where u.id = 1
