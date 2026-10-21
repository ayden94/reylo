insert into users (id, name) values (:id, :name) on conflict (id) do update set name = excluded.name returning old as was, new as now
