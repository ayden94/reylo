select * from (select count(*) as n from bar group by name) s
