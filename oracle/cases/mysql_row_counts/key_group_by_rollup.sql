select u.id, count(p.id) as n from users u left join posts p on p.user_id = u.id where u.id = :id group by u.id with rollup
