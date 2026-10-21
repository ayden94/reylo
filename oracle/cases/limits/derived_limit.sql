select s.name from (select * from bar where id = :id limit 5) s
