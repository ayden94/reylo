select a.n, u.email from users u right join (select count(*) as n from bar) a on u.id = :id
