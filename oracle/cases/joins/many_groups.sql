select u.email, count(*) as n from posts p left join users u on u.id = :id and u.id = p.author_id group by u.email
