select (select s.n from (select count(t.id) as n from bar b where false) s) as n from tags t
