select name, count(*) as n from users where id = :id group by name
