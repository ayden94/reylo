select t.id, (select count(b.id) from bar b where b.id = t.id) as n from tags t
