with ins as (insert into bar (id, name) values (:id, :name) returning id) select ins.id, c.n from ins, (select count(*) as n from bar) c
