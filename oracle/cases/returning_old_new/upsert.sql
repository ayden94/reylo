insert into users (id, name) values (:id, :name) on conflict (id) do update set name = excluded.name returning old.name as old_name, new.name as new_name
