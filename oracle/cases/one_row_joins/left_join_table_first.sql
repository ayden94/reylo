select a.n, u.email from users u left join (select count(*) as n from bar) a on true where u.id = :id
