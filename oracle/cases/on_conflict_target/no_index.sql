insert into bar (id, name) values (:id, :name) on conflict (name) do nothing
