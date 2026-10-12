with input as (select '%' || cast(:q as text) || '%' as pattern)
select id, name from users, input
where name like input.pattern or email like input.pattern
