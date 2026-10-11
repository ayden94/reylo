select u.id, (with mine as (select * from posts p where p.user_id = u.id) select count(*) from mine) as n from users u
