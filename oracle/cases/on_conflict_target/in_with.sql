with ins as (insert into bar (id, name) values (:id, :name) on conflict (name) do nothing returning id) select id from ins
