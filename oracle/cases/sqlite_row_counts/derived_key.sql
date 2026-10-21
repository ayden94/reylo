select s.name from (select name from users where id = :id limit 5) s
