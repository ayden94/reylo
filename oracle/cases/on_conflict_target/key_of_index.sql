insert into bar (id, code) values (:id, :code) on conflict (code) do nothing
