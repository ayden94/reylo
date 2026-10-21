select a.n, u.name from (select count(*) as n from users) a left join users u on u.name = :name
