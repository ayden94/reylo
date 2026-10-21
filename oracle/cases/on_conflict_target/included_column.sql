insert into bar (id, code, name) values (:id, :code, :name) on conflict (code, name) do nothing
