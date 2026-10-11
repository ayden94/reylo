insert into bar (id, name) values (:id, :name) on conflict (id) do nothing
