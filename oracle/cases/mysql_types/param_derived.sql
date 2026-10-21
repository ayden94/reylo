select t.x, t.y from (select :n + 1 as x, coalesce(:m, 0) as y) t
