select t.a, t.b from (select max(name) as b, count(*) as a from bar where false) t (a, b)
