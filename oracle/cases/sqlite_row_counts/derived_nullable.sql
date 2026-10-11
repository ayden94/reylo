select s.top, s.n from (select max(email) as top, count(*) as n from users where 0) s
