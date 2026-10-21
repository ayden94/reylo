with input as (select cast(:min as integer) as min)
select u.id, u.name, p.title, input.min as min_score
from input, users u left join posts p on p.user_id = u.id
where u.name like :q and u.score >= input.min
order by u.id limit :n offset :o
