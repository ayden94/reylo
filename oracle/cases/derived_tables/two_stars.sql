select s.*, t.* from (select count(*) as n from bar) s, (select max(name) as top from bar where false) t
