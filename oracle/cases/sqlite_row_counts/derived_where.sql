select s.n from (select count(*) as n from users) s where s.n > :min
