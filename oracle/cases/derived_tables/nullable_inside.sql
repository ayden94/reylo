select * from (select max(name) as top, count(*) as n from bar where false) s
