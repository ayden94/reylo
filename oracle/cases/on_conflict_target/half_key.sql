insert into bar (id, a, b) values (:id, :a, :b) on conflict (a) do nothing
