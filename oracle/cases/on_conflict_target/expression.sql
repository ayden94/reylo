insert into bar (id, name) values (:id, :name) on conflict (lower(name)) do nothing
