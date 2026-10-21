select * from (select count(*) as n from bar) s where s.n > :min
