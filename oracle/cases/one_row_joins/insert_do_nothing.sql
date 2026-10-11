with ins as (insert into bar (id, name) values (:id, :name) on conflict (id) do nothing returning id) select ins.id, c.n from ins, (select count(*) as n from bar) c
