select a.n, u.email from (select count(*) as n from bar) a left join users u on u.name is not null
