select a.n, b.id from (select count(*) as n from bar) a, (select id from bar where id = :id) b
