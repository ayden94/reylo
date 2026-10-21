with active as (select id, name from users where score >= 0) select a.id, a.name, (select count(*) from posts p where p.user_id = a.id) as n from active a
