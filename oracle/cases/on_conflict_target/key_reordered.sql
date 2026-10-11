insert into bar (id, a, b) values (:id, :a, :b) on conflict (b, a) do nothing
