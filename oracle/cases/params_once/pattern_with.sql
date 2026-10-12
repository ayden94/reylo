with input as (select concat('%', :q::text, '%') as pattern)
select id, name from users, input
where name like input.pattern or email like input.pattern
