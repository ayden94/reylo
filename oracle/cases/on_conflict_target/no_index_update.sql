insert into bar (id, name) values (:id, :name) on conflict (name) do update set name = excluded.name returning id
