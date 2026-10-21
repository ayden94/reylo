select id, count(*) as n from users where id = :id group by id with rollup
