select u.id, u.name, p.title, cast(:min as integer) as min_score
from users u left join posts p on p.user_id = u.id
where u.name like :q and u.score >= :min
order by u.id limit :n offset :o
