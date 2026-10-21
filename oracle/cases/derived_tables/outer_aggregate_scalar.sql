select (select count(t.id) from bar b where false) as n from tags t
