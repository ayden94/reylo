select name from (select * from bar limit 5) b where id = :id
