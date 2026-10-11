insert into plain (id, name) values (:id, :name) on conflict (id) do nothing
