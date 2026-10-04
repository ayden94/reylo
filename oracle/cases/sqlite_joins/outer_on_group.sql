select u.name, count(*) as n from posts p left join users u on u.id = :id and u.id = p.user_id group by u.name
