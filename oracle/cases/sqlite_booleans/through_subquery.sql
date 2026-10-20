select t.done, t.y from (select done, n > 0 as y from todos) t
